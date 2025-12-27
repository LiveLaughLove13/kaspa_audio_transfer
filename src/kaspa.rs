use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_consensus_core::constants::{SOMPI_PER_KASPA, UNACCEPTED_DAA_SCORE, MAX_TX_IN_SEQUENCE_NUM, TX_VERSION};

use crate::error::{AudioTransferError, Result};
use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::config::params::Params;
use kaspa_consensus_core::mass::MassCalculator;
use kaspa_consensus_core::network::NetworkId;
use kaspa_consensus_core::sign;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{PopulatedTransaction, Transaction, TransactionId, TransactionInput, TransactionOutpoint, TransactionOutput, UtxoEntry};
use kaspa_grpc_client::GrpcClient;
use kaspa_rpc_core::model::{RpcHash, RpcTransactionId};
use kaspa_rpc_core::notify::mode::NotificationMode;
use kaspa_txscript::pay_to_address_script;
use rand::RngCore;
use secp256k1::Keypair;
use std::collections::HashSet;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

const DEFAULT_KASPA_RPC_URL: &str = "grpc://127.0.0.1:16110";
const KAT_MAGIC: &[u8; 4] = b"KAT1";
const KAT_TYPE_MANIFEST: u8 = 1;
const KAT_TYPE_CHUNK: u8 = 2;
const MAX_STANDARD_COMPUTE_MASS: u64 = 100_000;
const MAX_STANDARD_TRANSIENT_MASS: u64 = 100_000;
const MAX_STANDARD_STORAGE_MASS: u64 = 100_000;
const MAX_CHUNK_DATA_SIZE: usize = 20_000;

type FileId = [u8; 16];

#[derive(Debug, Clone)]
struct KatManifest {
    file_id: FileId,
    total_size: u64,
    chunk_size: u32,
    total_chunks: u32,
}

pub struct KaspaClient {
    client: GrpcClient,
}

impl KaspaClient {
    pub async fn new(rpc_url: Option<&str>) -> Result<Self> {
        let rpc_url = rpc_url.unwrap_or(DEFAULT_KASPA_RPC_URL);
        eprintln!("Connecting to Kaspa node at: {}", rpc_url);
        let client = GrpcClient::connect_with_args(
            NotificationMode::Direct,
            rpc_url.to_string(),
            None,
            false,
            None,
            false,
            Some(180_000),
            Default::default(),
        )
        .await
        .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        let _ = client
            .get_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Failed to connect to Kaspa node: {e}")))?;
        eprintln!("Successfully connected to Kaspa node");
        Ok(Self { client })
    }

    fn network_prefix(&self, network: NetworkId) -> Prefix {
        network.network_type().into()
    }

    fn params_for_network(network: NetworkId) -> Params {
        Params::from(network)
    }

    fn parse_private_key_hex(hex_str: &str) -> Result<[u8; 32]> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid private key hex: {e}")))?;
        if bytes.len() != 32 {
            return Err(AudioTransferError::InvalidInput(
                "Private key must be 32 bytes (64 hex chars)".to_string(),
            ));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(out)
    }

    fn sompi_to_kas(v: u64) -> f64 {
        v as f64 / SOMPI_PER_KASPA as f64
    }

    async fn feerate_priority(&self) -> Result<f64> {
        let est = self
            .client
            .get_fee_estimate()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        Ok(est.priority_bucket.feerate)
    }

    pub async fn estimate_audio_fees(&self, audio_data: &[u8], from_private_key_hex: &str, amount: f64) -> Result<()> {
        let send_value = (amount * SOMPI_PER_KASPA as f64) as u64;
        if send_value == 0 {
            return Err(AudioTransferError::InvalidInput(
                "amount must be > 0 (in KAS)".to_string(),
            ));
        }

        let dag = self
            .client
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        let network_id = dag.network;
        let prefix = self.network_prefix(network_id);

        let privkey = Self::parse_private_key_hex(from_private_key_hex)?;
        let keypair = Keypair::from_seckey_slice(secp256k1::SECP256K1, &privkey)
            .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid private key: {e}")))?;
        let xonly_pk = keypair.public_key().x_only_public_key().0;
        let from_addr = Address::new(prefix, Version::PubKey, &xonly_pk.serialize());

        let self_spk = pay_to_address_script(&from_addr);
        let to_spk = self_spk.clone();

        let params = Self::params_for_network(network_id);
        let mass_calc = MassCalculator::new(
            params.mass_per_tx_byte,
            params.mass_per_script_pub_key_byte,
            params.mass_per_sig_op,
            params.storage_mass_parameter,
        );

        let mut utxos = self
            .client
            .get_utxos_by_addresses(vec![from_addr.clone()])
            .await
            .map_err(|e| {
                AudioTransferError::KaspaRpc(format!(
                    "get_utxos_by_addresses failed (node needs --utxoindex): {e}"
                ))
            })?;
        if utxos.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "No UTXOs found for from_private_key".to_string(),
            ));
        }
        utxos.sort_by_key(|u| std::cmp::Reverse(u.utxo_entry.amount));
        let picked = utxos.into_iter().next().unwrap();
        let initial_entry_amount = picked.utxo_entry.amount;
        let initial_entry_spk = picked.utxo_entry.script_public_key.clone();

        let chunk_overhead = 4 + 1 + 16 + 4 + 4 + 4;
        let chunk_template = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(
                TransactionOutpoint::new(TransactionId::default(), 0),
                vec![0u8; 66],
                MAX_TX_IN_SEQUENCE_NUM,
                1,
            )],
            vec![TransactionOutput::new(initial_entry_amount.max(1), self_spk.clone())],
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            vec![],
        );
        let max_payload_chunk = Self::compute_max_payload_len_for_template(
            &mass_calc,
            chunk_template,
            initial_entry_amount,
            initial_entry_spk,
        );
        if max_payload_chunk <= chunk_overhead + 1 {
            return Err(AudioTransferError::KaspaRpc(
                "Unable to compute safe chunk payload size".to_string(),
            ));
        }
        let chunk_data_size = (max_payload_chunk - chunk_overhead).min(MAX_CHUNK_DATA_SIZE);
        let total_chunks = ((audio_data.len() + chunk_data_size - 1) / chunk_data_size) as u32;

        println!(
            "Chunking file into {} chunk(s) of up to {} bytes each",
            total_chunks, chunk_data_size
        );

        let mut file_id: FileId = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut file_id);
        let manifest = KatManifest {
            file_id,
            total_size: audio_data.len() as u64,
            chunk_size: chunk_data_size as u32,
            total_chunks,
        };
        let manifest_payload = Self::encode_manifest_payload(&manifest);

        let feerate = self.feerate_priority().await?;
        let worst_chunk_payload_len = (4 + 1 + 16 + 4 + 4 + 4) + chunk_data_size;

        let mut assumed_input_amount = initial_entry_amount.max(1);
        let mut manifest_fee: u64 = 0;
        let mut chunk_fees: u64 = 0;

        // Try a small number of times with exponential backoff.
        for _ in 0..32u32 {
            let (fee, change) = Self::estimate_manifest_fee_and_change(
                &mass_calc,
                feerate,
                assumed_input_amount,
                send_value,
                &self_spk,
                &to_spk,
                manifest_payload.clone(),
            )?;

            if change == 0 {
                assumed_input_amount = assumed_input_amount
                    .saturating_mul(2)
                    .max(assumed_input_amount.saturating_add(SOMPI_PER_KASPA));
                continue;
            }

            match Self::estimate_total_chunk_fees(
                &mass_calc,
                feerate,
                change,
                total_chunks,
                worst_chunk_payload_len,
                &self_spk,
            ) {
                Ok((fees, _final_amount)) => {
                    manifest_fee = fee;
                    chunk_fees = fees;
                    break;
                }
                Err(AudioTransferError::KaspaRpc(msg))
                    if msg.to_lowercase().contains("insufficient funds for chunk fees") =>
                {
                    assumed_input_amount = assumed_input_amount
                        .saturating_mul(2)
                        .max(assumed_input_amount.saturating_add(SOMPI_PER_KASPA));
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        if manifest_fee == 0 && chunk_fees == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Unable to compute fee estimate (could not find an assumed input amount that satisfies the fee model)".to_string(),
            ));
        }

        if assumed_input_amount > initial_entry_amount {
            println!(
                "Warning: your largest available UTXO ({:.8} KAS) appears insufficient for amount+fees; estimate assumes {:.8} KAS input",
                Self::sompi_to_kas(initial_entry_amount),
                Self::sompi_to_kas(assumed_input_amount)
            );
        }

        let total_fee = manifest_fee.saturating_add(chunk_fees);
        println!("Manifest: {:.8} KAS", Self::sompi_to_kas(manifest_fee));
        println!("Chunks: {:.8} KAS", Self::sompi_to_kas(chunk_fees));
        println!("Total: {:.8} KAS", Self::sompi_to_kas(total_fee));

        if !audio_data.is_empty() {
            let mib = audio_data.len() as f64 / (1024.0 * 1024.0);
            if mib > 0.0 {
                let eff = Self::sompi_to_kas(total_fee) / mib;
                println!("Effective cost: {:.8} KAS per MiB", eff);
            }
        }

        Ok(())
    }

    pub async fn send_audio_signed(
        &self,
        audio_data: &[u8],
        from_private_key_hex: &str,
        to_address: &str,
        amount: f64,
        resume_from: Option<&str>,
        resume_output_index: u32,
    ) -> Result<String> {
        let send_value = (amount * SOMPI_PER_KASPA as f64) as u64;
        if send_value == 0 && resume_from.is_none() {
            return Err(AudioTransferError::InvalidInput(
                "amount must be > 0 (in KAS)".to_string(),
            ));
        }

        println!(
            "Preparing to send {} KAS to {} with {} bytes of payload",
            amount,
            to_address,
            audio_data.len()
        );

        let dag = self
            .client
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        let network_id = dag.network;
        let prefix = self.network_prefix(network_id);

        let to_addr = Address::try_from(to_address)
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid to_address: {e}")))?;
        if to_addr.prefix != prefix {
            return Err(AudioTransferError::InvalidInput(format!(
                "Address prefix mismatch. Node network prefix is '{prefix}', but to_address is '{}'",
                to_addr.prefix
            )));
        }

        let privkey = Self::parse_private_key_hex(from_private_key_hex)?;
        let keypair = Keypair::from_seckey_slice(secp256k1::SECP256K1, &privkey)
            .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid private key: {e}")))?;
        let xonly_pk = keypair.public_key().x_only_public_key().0;
        let from_addr = Address::new(prefix, Version::PubKey, &xonly_pk.serialize());

        let self_spk = pay_to_address_script(&from_addr);
        let to_spk = pay_to_address_script(&to_addr);

        let params = Self::params_for_network(network_id);
        let mass_calc = MassCalculator::new(
            params.mass_per_tx_byte,
            params.mass_per_script_pub_key_byte,
            params.mass_per_sig_op,
            params.storage_mass_parameter,
        );

        let (initial_outpoint, initial_entry_amount, initial_entry_spk) =
            self.resolve_initial_utxo(&from_addr, resume_from, resume_output_index).await?;

        let chunk_overhead = 4 + 1 + 16 + 4 + 4 + 4;
        let chunk_template = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(initial_outpoint, Vec::new(), MAX_TX_IN_SEQUENCE_NUM, 1)],
            vec![TransactionOutput::new(initial_entry_amount.max(1), self_spk.clone())],
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            vec![],
        );
        let max_payload_chunk = Self::compute_max_payload_len_for_template(
            &mass_calc,
            chunk_template,
            initial_entry_amount,
            initial_entry_spk.clone(),
        );
        if max_payload_chunk <= chunk_overhead + 1 {
            return Err(AudioTransferError::KaspaRpc("Unable to compute safe chunk payload size".to_string()));
        }
        let chunk_data_size = (max_payload_chunk - chunk_overhead).min(MAX_CHUNK_DATA_SIZE);
        let total_chunks = ((audio_data.len() + chunk_data_size - 1) / chunk_data_size) as u32;

        println!(
            "Chunking file into {} chunk(s) of up to {} bytes each",
            total_chunks, chunk_data_size
        );

        let mut file_id: FileId = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut file_id);
        let manifest = KatManifest {
            file_id,
            total_size: audio_data.len() as u64,
            chunk_size: chunk_data_size as u32,
            total_chunks,
        };

        let feerate = self.feerate_priority().await?;
        let (manifest_fee, manifest_change) = Self::estimate_manifest_fee_and_change(
            &mass_calc,
            feerate,
            initial_entry_amount,
            send_value,
            &self_spk,
            &to_spk,
            Self::encode_manifest_payload(&manifest),
        )?;
        if manifest_change == 0 {
            return Err(AudioTransferError::KaspaRpc("Insufficient funds for manifest tx".to_string()));
        }

        let mut outputs = Vec::new();
        if send_value > 0 {
            outputs.push(TransactionOutput::new(send_value, to_spk.clone()));
        }
        outputs.push(TransactionOutput::new(manifest_change.max(1), self_spk.clone()));

        let mut manifest_tx = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(initial_outpoint, Vec::new(), MAX_TX_IN_SEQUENCE_NUM, 1)],
            outputs,
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            Self::encode_manifest_payload(&manifest),
        );
        manifest_tx.inputs[0].signature_script = vec![0u8; 66];
        manifest_tx.finalize();

        let populated = PopulatedTransaction::new(
            &manifest_tx,
            vec![kaspa_consensus_core::tx::UtxoEntry::new(
                initial_entry_amount,
                initial_entry_spk.clone(),
                UNACCEPTED_DAA_SCORE,
                false,
            )],
        );
        let non_ctx = mass_calc.calc_non_contextual_masses(&manifest_tx);
        if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS {
            return Err(AudioTransferError::KaspaRpc(format!(
                "Manifest not standard (compute_mass={}, transient_mass={})",
                non_ctx.compute_mass, non_ctx.transient_mass
            )));
        }
        let ctx = mass_calc
            .calc_contextual_masses(&populated)
            .ok_or_else(|| AudioTransferError::KaspaRpc("Mass incomputable".to_string()))?;
        if ctx.storage_mass > MAX_STANDARD_STORAGE_MASS {
            return Err(AudioTransferError::KaspaRpc(format!(
                "Manifest not standard (storage_mass={} > {})",
                ctx.storage_mass, MAX_STANDARD_STORAGE_MASS
            )));
        }
        manifest_tx.set_mass(ctx.storage_mass);
        manifest_tx = Self::finalize_and_sign_single_input(
            manifest_tx,
            kaspa_consensus_core::tx::UtxoEntry::new(initial_entry_amount, initial_entry_spk.clone(), UNACCEPTED_DAA_SCORE, false),
            &keypair,
        )?;

        let manifest_txid: RpcTransactionId = self
            .client
            .submit_transaction((&manifest_tx).into(), false)
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        println!("Submitted manifest tx: {manifest_txid}");

        let change_index = if send_value > 0 { 1 } else { 0 };
        let mut next_outpoint = TransactionOutpoint::new(manifest_tx.id(), change_index);
        let mut next_entry = kaspa_consensus_core::tx::UtxoEntry::new(manifest_change, self_spk.clone(), UNACCEPTED_DAA_SCORE, false);
        let mut chain_value = manifest_change;
        let mut total_chunk_fees: u64 = 0;

        for chunk_index in 0..total_chunks {
            let start = (chunk_index as usize) * chunk_data_size;
            let end = (start + chunk_data_size).min(audio_data.len());
            let data = &audio_data[start..end];
            let payload = Self::encode_chunk_payload(&manifest.file_id, chunk_index, total_chunks, data);

            let mut tx = Transaction::new(
                TX_VERSION,
                vec![TransactionInput::new(next_outpoint, Vec::new(), MAX_TX_IN_SEQUENCE_NUM, 1)],
                vec![TransactionOutput::new(chain_value.max(1), self_spk.clone())],
                0,
                SUBNETWORK_ID_NATIVE,
                0,
                payload,
            );

            let (chunk_fee, out_value) = Self::estimate_single_output_fee_and_out(
                &mass_calc,
                feerate,
                chain_value,
                &self_spk,
                vec![0u8; tx.payload.len()],
            )?;
            total_chunk_fees = total_chunk_fees.saturating_add(chunk_fee);
            if out_value == 0 {
                return Err(AudioTransferError::KaspaRpc("Insufficient funds for chunk fee".to_string()));
            }

            tx.outputs[0].value = out_value;

            tx.inputs[0].signature_script = vec![0u8; 66];
            tx.finalize();

            let populated = PopulatedTransaction::new(&tx, vec![next_entry.clone()]);
            let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
            if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS {
                return Err(AudioTransferError::KaspaRpc(format!(
                    "Chunk {chunk_index} not standard (compute_mass={}, transient_mass={})",
                    non_ctx.compute_mass, non_ctx.transient_mass
                )));
            }
            let ctx = mass_calc
                .calc_contextual_masses(&populated)
                .ok_or_else(|| AudioTransferError::KaspaRpc("Mass incomputable".to_string()))?;
            if ctx.storage_mass > MAX_STANDARD_STORAGE_MASS {
                return Err(AudioTransferError::KaspaRpc(format!(
                    "Chunk {chunk_index} not standard (storage_mass={} > {})",
                    ctx.storage_mass, MAX_STANDARD_STORAGE_MASS
                )));
            }
            tx.set_mass(ctx.storage_mass);
            tx = Self::finalize_and_sign_single_input(tx, next_entry.clone(), &keypair)?;

            let submitted: RpcTransactionId = self
                .client
                .submit_transaction((&tx).into(), false)
                .await
                .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

            next_outpoint = TransactionOutpoint::new(tx.id(), 0);
            chain_value = out_value;
            next_entry = kaspa_consensus_core::tx::UtxoEntry::new(out_value, self_spk.clone(), UNACCEPTED_DAA_SCORE, false);

            if (chunk_index + 1) % 10 == 0 || chunk_index + 1 == total_chunks {
                println!("Submitted chunk {}/{}: {}", chunk_index + 1, total_chunks, submitted);
            }
        }

        let total_fee_sompi = total_chunk_fees.saturating_add(manifest_fee);
        println!(
            "Fee summary: manifest {:.8} KAS, chunks {:.8} KAS, total {:.8} KAS",
            Self::sompi_to_kas(manifest_fee),
            Self::sompi_to_kas(total_chunk_fees),
            Self::sompi_to_kas(total_fee_sompi)
        );

        Ok(manifest_txid.to_string())
    }

    async fn resolve_initial_utxo(
        &self,
        from_addr: &Address,
        resume_from: Option<&str>,
        resume_output_index: u32,
    ) -> Result<(TransactionOutpoint, u64, kaspa_consensus_core::tx::ScriptPublicKey)> {
        if let Some(resume_txid_str) = resume_from {
            let resume_txid = TransactionId::from_str(resume_txid_str)
                .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid resume_from txid: {e}")))?;
            let outpoint = TransactionOutpoint::new(resume_txid, resume_output_index);
            let idx = resume_output_index as usize;

            let mempool_out = self
                .client
                .get_mempool_entry(resume_txid, true, false)
                .await
                .ok()
                .and_then(|e| e.transaction.outputs.get(idx).cloned());

            if let Some(out) = mempool_out {
                return Ok((outpoint, out.value, out.script_public_key));
            }

            let utxos = self
                .client
                .get_utxos_by_addresses(vec![from_addr.clone()])
                .await
                .map_err(|e| {
                    AudioTransferError::KaspaRpc(format!(
                        "get_utxos_by_addresses failed (node needs --utxoindex): {e}"
                    ))
                })?;

            let found = utxos
                .into_iter()
                .find(|u| u.outpoint.transaction_id == resume_txid && u.outpoint.index == resume_output_index)
                .ok_or_else(|| {
                    AudioTransferError::KaspaRpc(format!(
                        "resume_from outpoint ({}, {}) not found in mempool or UTXO set. It may be spent or the txid/index is wrong.",
                        resume_txid, resume_output_index
                    ))
                })?;

            return Ok((
                outpoint,
                found.utxo_entry.amount,
                found.utxo_entry.script_public_key,
            ));
        }

        // Gather all outpoints currently spent by mempool transactions from this address
        let mut mempool_spent: HashSet<TransactionOutpoint> = HashSet::new();
        if let Ok(mut by_addr) = self
            .client
            .get_mempool_entries_by_addresses(vec![from_addr.clone()], true, false)
            .await
        {
            if let Some(entry) = by_addr.pop() {
                for e in entry.sending {
                    for i in e.transaction.inputs {
                        mempool_spent.insert(TransactionOutpoint::new(
                            i.previous_outpoint.transaction_id,
                            i.previous_outpoint.index,
                        ));
                    }
                }
            }
        }

        let mut utxos = self
            .client
            .get_utxos_by_addresses(vec![from_addr.clone()])
            .await
            .map_err(|e| {
                AudioTransferError::KaspaRpc(format!(
                    "get_utxos_by_addresses failed (node needs --utxoindex): {e}"
                ))
            })?;

        if utxos.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "No UTXOs found for from_private_key".to_string(),
            ));
        }

        if !mempool_spent.is_empty() {
            utxos.retain(|u| {
                let o = TransactionOutpoint::new(u.outpoint.transaction_id, u.outpoint.index);
                !mempool_spent.contains(&o)
            });
        }

        if utxos.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "All available UTXOs are currently being spent by mempool transactions. Wait for confirmations and try again.".to_string(),
            ));
        }

        utxos.sort_by_key(|u| std::cmp::Reverse(u.utxo_entry.amount));
        let picked = utxos.into_iter().next().unwrap();
        Ok((
            TransactionOutpoint::new(picked.outpoint.transaction_id, picked.outpoint.index),
            picked.utxo_entry.amount,
            picked.utxo_entry.script_public_key,
        ))
    }

    fn compute_max_payload_len_for_template(
        mass_calc: &MassCalculator,
        mut tx: Transaction,
        input_amount: u64,
        input_spk: kaspa_consensus_core::tx::ScriptPublicKey,
    ) -> usize {
        if !tx.inputs.is_empty() {
            tx.inputs[0].signature_script = vec![0u8; 66];
            tx.inputs[0].sig_op_count = 1;
        }
        tx.finalize();

        let base_non_ctx = mass_calc.calc_non_contextual_masses(&tx);
        if base_non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS || base_non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS {
            return 0;
        }
        if input_amount == 0 {
            return 0;
        }
        let entry = UtxoEntry::new(input_amount, input_spk.clone(), UNACCEPTED_DAA_SCORE, false);
        let populated = PopulatedTransaction::new(&tx, vec![entry]);
        if let Some(ctx) = mass_calc.calc_contextual_masses(&populated) {
            if ctx.storage_mass > MAX_STANDARD_STORAGE_MASS {
                return 0;
            }
        } else {
            return 0;
        }

        let mut lo: usize = 0;
        let mut hi: usize = 200_000;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            tx.payload = vec![0u8; mid];
            tx.finalize();

            let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
            if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS {
                hi = mid - 1;
                continue;
            }

            let entry = UtxoEntry::new(input_amount, input_spk.clone(), UNACCEPTED_DAA_SCORE, false);
            let populated = PopulatedTransaction::new(&tx, vec![entry]);
            let Some(ctx) = mass_calc.calc_contextual_masses(&populated) else {
                hi = mid - 1;
                continue;
            };
            if ctx.storage_mass > MAX_STANDARD_STORAGE_MASS {
                hi = mid - 1;
                continue;
            }

            lo = mid;
        }
        lo
    }

    fn is_kat_payload(payload: &[u8]) -> bool {
        payload.len() >= 5 && payload[0..4] == *KAT_MAGIC
    }

    fn encode_manifest_payload(manifest: &KatManifest) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 1 + 16 + 8 + 4 + 4);
        out.extend_from_slice(KAT_MAGIC);
        out.push(KAT_TYPE_MANIFEST);
        out.extend_from_slice(&manifest.file_id);
        out.extend_from_slice(&manifest.total_size.to_le_bytes());
        out.extend_from_slice(&manifest.chunk_size.to_le_bytes());
        out.extend_from_slice(&manifest.total_chunks.to_le_bytes());
        out
    }

    fn decode_manifest_payload(payload: &[u8]) -> Result<KatManifest> {
        if payload.len() < 37 {
            return Err(AudioTransferError::InvalidInput(
                "Invalid manifest payload".to_string(),
            ));
        }
        if payload[0..4] != *KAT_MAGIC || payload[4] != KAT_TYPE_MANIFEST {
            return Err(AudioTransferError::InvalidInput(
                "Invalid manifest payload".to_string(),
            ));
        }
        let mut file_id: FileId = [0u8; 16];
        file_id.copy_from_slice(&payload[5..21]);
        let total_size = u64::from_le_bytes(payload[21..29].try_into().unwrap());
        let chunk_size = u32::from_le_bytes(payload[29..33].try_into().unwrap());
        let total_chunks = u32::from_le_bytes(payload[33..37].try_into().unwrap());
        Ok(KatManifest {
            file_id,
            total_size,
            chunk_size,
            total_chunks,
        })
    }

    fn encode_chunk_payload(file_id: &FileId, idx: u32, total: u32, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + 1 + 16 + 4 + 4 + 4 + data.len());
        out.extend_from_slice(KAT_MAGIC);
        out.push(KAT_TYPE_CHUNK);
        out.extend_from_slice(file_id);
        out.extend_from_slice(&idx.to_le_bytes());
        out.extend_from_slice(&total.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(data);
        out
    }

    fn try_decode_chunk_header(payload: &[u8]) -> Option<(FileId, u32, u32, usize)> {
        if payload.len() < 33 {
            return None;
        }
        if payload[0..4] != *KAT_MAGIC || payload[4] != KAT_TYPE_CHUNK {
            return None;
        }
        let mut file_id: FileId = [0u8; 16];
        file_id.copy_from_slice(&payload[5..21]);
        let idx = u32::from_le_bytes(payload[21..25].try_into().ok()?);
        let total = u32::from_le_bytes(payload[25..29].try_into().ok()?);
        Some((file_id, idx, total, 33))
    }

    fn estimate_tx_fee_sompi(
        mass_calc: &MassCalculator,
        tx: &Transaction,
        entry: &UtxoEntry,
        feerate_sompi_per_gram: f64,
    ) -> Result<u64> {
        if entry.amount == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Cannot estimate fee with 0-value input".to_string(),
            ));
        }
        if tx.outputs.iter().any(|o| o.value == 0) {
            return Err(AudioTransferError::KaspaRpc(
                "Cannot estimate fee with 0-value output".to_string(),
            ));
        }
        let non_ctx = mass_calc.calc_non_contextual_masses(tx);
        if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS {
            return Err(AudioTransferError::KaspaRpc(format!(
                "Transaction too large for standard tx (compute_mass={})",
                non_ctx.compute_mass
            )));
        }
        let populated = PopulatedTransaction::new(tx, vec![entry.clone()]);
        let ctx = mass_calc
            .calc_contextual_masses(&populated)
            .ok_or_else(|| AudioTransferError::KaspaRpc("Mass incomputable".to_string()))?;
        let total_mass = non_ctx
            .compute_mass
            .max(non_ctx.transient_mass)
            .max(ctx.storage_mass);
        Ok((total_mass as f64 * feerate_sompi_per_gram).ceil() as u64)
    }

    fn estimate_manifest_fee_and_change(
        mass_calc: &MassCalculator,
        feerate_sompi_per_gram: f64,
        input_amount: u64,
        send_value: u64,
        self_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
        to_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
        payload: Vec<u8>,
    ) -> Result<(u64, u64)> {
        if input_amount == 0 {
            return Ok((0, 0));
        }
        if send_value > input_amount {
            return Ok((0, 0));
        }

        // Iteratively solve fee/change since storage mass depends on output amounts.
        let mut change_guess: u64 = input_amount.saturating_sub(send_value).max(1);
        let mut last_fee: u64 = 0;

        for _ in 0..20u32 {
            let mut outputs = Vec::new();
            if send_value > 0 {
                outputs.push(TransactionOutput::new(send_value, to_spk.clone()));
            }
            outputs.push(TransactionOutput::new(change_guess, self_spk.clone()));

            let mut tx = Transaction::new(
                TX_VERSION,
                vec![TransactionInput::new(
                    TransactionOutpoint::new(TransactionId::default(), 0),
                    vec![0u8; 66],
                    MAX_TX_IN_SEQUENCE_NUM,
                    1,
                )],
                outputs,
                0,
                SUBNETWORK_ID_NATIVE,
                0,
                payload.clone(),
            );
            tx.finalize();

            let entry = UtxoEntry::new(input_amount, self_spk.clone(), UNACCEPTED_DAA_SCORE, false);
            let fee = Self::estimate_tx_fee_sompi(mass_calc, &tx, &entry, feerate_sompi_per_gram)?;
            let required = send_value.saturating_add(fee);
            let new_change = input_amount.saturating_sub(required);

            if new_change == 0 {
                return Ok((fee, 0));
            }
            if new_change == change_guess && fee == last_fee {
                return Ok((fee, new_change));
            }

            last_fee = fee;
            change_guess = new_change;
        }

        // Return last computed values
        let required = send_value.saturating_add(last_fee);
        let final_change = input_amount.saturating_sub(required);
        Ok((last_fee, final_change))
    }

    fn estimate_single_output_fee_and_out(
        mass_calc: &MassCalculator,
        feerate_sompi_per_gram: f64,
        input_amount: u64,
        self_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
        payload_placeholder: Vec<u8>,
    ) -> Result<(u64, u64)> {
        if input_amount == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Insufficient funds for chunk fees".to_string(),
            ));
        }

        // Iteratively solve fee/out_value since storage mass depends on output amount.
        let mut out_guess: u64 = input_amount.max(1);
        let mut last_fee: u64 = 0;

        for _ in 0..20u32 {
            let mut tx = Transaction::new(
                TX_VERSION,
                vec![TransactionInput::new(
                    TransactionOutpoint::new(TransactionId::default(), 0),
                    vec![0u8; 66],
                    MAX_TX_IN_SEQUENCE_NUM,
                    1,
                )],
                vec![TransactionOutput::new(out_guess, self_spk.clone())],
                0,
                SUBNETWORK_ID_NATIVE,
                0,
                payload_placeholder.clone(),
            );
            tx.finalize();

            let entry = UtxoEntry::new(input_amount, self_spk.clone(), UNACCEPTED_DAA_SCORE, false);
            let fee = Self::estimate_tx_fee_sompi(mass_calc, &tx, &entry, feerate_sompi_per_gram)?;
            let new_out = input_amount.saturating_sub(fee);
            if new_out == 0 {
                return Err(AudioTransferError::KaspaRpc(
                    "Insufficient funds for chunk fees".to_string(),
                ));
            }

            if new_out == out_guess && fee == last_fee {
                return Ok((fee, new_out));
            }

            last_fee = fee;
            out_guess = new_out;
        }

        let final_out = input_amount.saturating_sub(last_fee);
        if final_out == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Insufficient funds for chunk fees".to_string(),
            ));
        }
        Ok((last_fee, final_out))
    }

    fn estimate_total_chunk_fees(
        mass_calc: &MassCalculator,
        feerate_sompi_per_gram: f64,
        starting_amount: u64,
        total_chunks: u32,
        worst_payload_len: usize,
        self_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
    ) -> Result<(u64, u64)> {
        if starting_amount == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Insufficient funds for chunk fees".to_string(),
            ));
        }
        let mut fees: u64 = 0;
        let mut amount = starting_amount;
        for _ in 0..total_chunks {
            let (fee, out_value) = Self::estimate_single_output_fee_and_out(
                mass_calc,
                feerate_sompi_per_gram,
                amount,
                self_spk,
                vec![0u8; worst_payload_len],
            )?;
            fees = fees.saturating_add(fee);
            amount = out_value;
        }
        Ok((fees, amount))
    }

    fn finalize_and_sign_single_input(mut tx: Transaction, entry: UtxoEntry, keypair: &Keypair) -> Result<Transaction> {
        if tx.inputs.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "Transaction must have at least one input".to_string(),
            ));
        }
        tx.inputs[0].sig_op_count = 1;
        let mut signable = kaspa_consensus_core::tx::SignableTransaction::new(tx);
        signable.entries[0] = Some(entry);
        let mut signed = sign::sign(signable, keypair.clone());
        signed.tx.finalize();
        Ok(signed.tx)
    }

    async fn find_transaction_payload(&self, tx_id: TransactionId, start_block_hash: Option<&str>) -> Result<Vec<u8>> {
        if let Ok(entry) = self.client.get_mempool_entry(tx_id, true, false).await {
            return Ok(entry.transaction.payload);
        }

        let tx_id_str = tx_id.to_string();

        let dag = self
            .client
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        let mut start_hash: RpcHash = if let Some(h) = start_block_hash {
            RpcHash::from_str(h)
                .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid start_block_hash: {e}")))?
        } else {
            dag.pruning_point_hash
        };
        for _page in 0..2000u32 {
            let response = {
                let mut attempt: u32 = 0;
                let mut backoff_ms: u64 = 250;
                loop {
                    attempt += 1;
                    match self
                        .client
                        .get_virtual_chain_from_block(start_hash, true, None)
                        .await
                    {
                        Ok(resp) => break resp,
                        Err(e) => {
                            let msg = e.to_string();

                            let is_timeout = msg.to_lowercase().contains("timeout");
                            if !is_timeout || attempt >= 10 {
                                return Err(AudioTransferError::KaspaRpc(msg));
                            }
                            sleep(Duration::from_millis(backoff_ms)).await;
                            backoff_ms = (backoff_ms * 2).min(10_000);
                        }
                    }
                }
            };

            for bucket in response.accepted_transaction_ids.iter() {
                let is_target = bucket
                    .accepted_transaction_ids
                    .iter()
                    .any(|id| id.to_string() == tx_id_str);
                if !is_target {
                    continue;
                }

                let block = self
                    .client
                    .get_block(bucket.accepting_block_hash, true)
                    .await
                    .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

                for tx in block.transactions.iter() {
                    let is_target = tx
                        .verbose_data
                        .as_ref()
                        .is_some_and(|v| v.transaction_id.to_string() == tx_id_str);
                    if !is_target {
                        continue;
                    }
                    return Ok(tx.payload.clone());
                }
            }

            let added = &response.added_chain_block_hashes;
            let Some(last) = added.last().copied() else { break; };
            if last == start_hash {
                break;
            }
            start_hash = last;
        }

        Err(AudioTransferError::KaspaRpc(
            "Transaction not found in mempool or virtual chain scan".to_string(),
        ))
    }

    pub async fn receive_audio(&self, tx_id: &str, start_block_hash: Option<&str>) -> Result<Vec<u8>> {
        println!("Fetching transaction: {}", tx_id);
        let tx_id = TransactionId::from_str(tx_id)
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid transaction ID: {e}")))?;

        let payload = self.find_transaction_payload(tx_id, start_block_hash).await?;

        if !Self::is_kat_payload(&payload) {
            return Ok(payload);
        }

        if payload.len() >= 5 && payload[4] == KAT_TYPE_CHUNK {
            let Some((_file_id, _idx, _total, offset)) = Self::try_decode_chunk_header(&payload) else {
                return Err(AudioTransferError::InvalidInput("Invalid chunk payload".to_string()));
            };
            let data_len = u32::from_le_bytes(payload[29..33].try_into().unwrap()) as usize;
            return Ok(payload[offset..offset + data_len].to_vec());
        }

        let manifest = Self::decode_manifest_payload(&payload)?;
        println!(
            "Manifest: file_id={}, total_size={}, chunk_size={}, total_chunks={}",
            hex::encode(manifest.file_id),
            manifest.total_size,
            manifest.chunk_size,
            manifest.total_chunks
        );

        let mut chunks: Vec<Option<Vec<u8>>> = vec![None; manifest.total_chunks as usize];

        // mempool
        if let Ok(entries) = self
            .client
            .get_mempool_entries(true, false)
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))
        {
            for entry in entries {
                let p = entry.transaction.payload;
                if let Some((file_id, idx, total, offset)) = Self::try_decode_chunk_header(&p) {
                    if file_id == manifest.file_id && total == manifest.total_chunks {
                        let data_len = u32::from_le_bytes(p[29..33].try_into().unwrap()) as usize;
                        if (idx as usize) < chunks.len() {
                            chunks[idx as usize] = Some(p[offset..offset + data_len].to_vec());
                        }
                    }
                }
            }
        }

        // paged accepted scan
        let dag = self
            .client
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        let mut start_hash: RpcHash = if let Some(h) = start_block_hash {
            RpcHash::from_str(h)
                .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid start_block_hash: {e}")))?
        } else {
            dag.pruning_point_hash
        };

        for _page in 0..2000u32 {
            let response = {
                let mut attempt: u32 = 0;
                let mut backoff_ms: u64 = 250;
                loop {
                    attempt += 1;
                    match self
                        .client
                        .get_virtual_chain_from_block(start_hash, true, None)
                        .await
                    {
                        Ok(resp) => break resp,
                        Err(e) => {
                            let msg = e.to_string();
                            let is_timeout = msg.to_lowercase().contains("timeout");

                            if !is_timeout || attempt >= 10 {
                                return Err(AudioTransferError::KaspaRpc(msg));
                            }
                            sleep(Duration::from_millis(backoff_ms)).await;
                            backoff_ms = (backoff_ms * 2).min(10_000);
                        }
                    }
                }
            };

            for bucket in response.accepted_transaction_ids.iter() {
                if chunks.iter().all(|c| c.is_some()) {
                    break;
                }

                let block = self
                    .client
                    .get_block(bucket.accepting_block_hash, true)
                    .await
                    .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

                for tx in block.transactions.iter() {
                    let p = &tx.payload;
                    if let Some((file_id, idx, total, offset)) = Self::try_decode_chunk_header(p) {
                        if file_id == manifest.file_id && total == manifest.total_chunks {
                            let data_len = u32::from_le_bytes(p[29..33].try_into().unwrap()) as usize;
                            if offset + data_len <= p.len() && (idx as usize) < chunks.len() {
                                chunks[idx as usize] = Some(p[offset..offset + data_len].to_vec());
                            }
                        }
                    }
                }
            }

            if chunks.iter().all(|c| c.is_some()) {
                break;
            }

            let added = &response.added_chain_block_hashes;
            let Some(last) = added.last().copied() else { break; };
            if last == start_hash {
                break;
            }
            start_hash = last;
        }

        let mut out = Vec::with_capacity(manifest.total_size as usize);
        for i in 0..(manifest.total_chunks as usize) {
            let Some(c) = chunks[i].as_ref() else {
                return Err(AudioTransferError::KaspaRpc(format!("Missing chunk {i}")));
            };
            out.extend_from_slice(c);
        }
        out.truncate(manifest.total_size as usize);
        Ok(out)
    }

    pub async fn get_tx_accepting_block_hash(
        &self,
        tx_id: &str,
        start_block_hash: Option<&str>,
        _min_confirmation_count: u64,
    ) -> Result<Option<String>> {
        let tx_id = TransactionId::from_str(tx_id)
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid transaction ID: {e}")))?;

        if self.client.get_mempool_entry(tx_id, true, false).await.is_ok() {
            return Ok(None);
        }

        let tx_id_str = tx_id.to_string();

        let dag = self
            .client
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        let mut start_hash: RpcHash = if let Some(h) = start_block_hash {
            RpcHash::from_str(h)
                .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid start_block_hash: {e}")))?
        } else {
            dag.pruning_point_hash
        };

        for _page in 0..2000u32 {
            let response = {
                let mut attempt: u32 = 0;
                let mut backoff_ms: u64 = 250;
                loop {
                    attempt += 1;
                    match self
                        .client
                        .get_virtual_chain_from_block(start_hash, true, None)
                        .await
                    {
                        Ok(resp) => break resp,
                        Err(e) => {
                            let msg = e.to_string();
                            let is_timeout = msg.to_lowercase().contains("timeout");
                            if !is_timeout || attempt >= 10 {
                                return Err(AudioTransferError::KaspaRpc(msg));
                            }
                            sleep(Duration::from_millis(backoff_ms)).await;
                            backoff_ms = (backoff_ms * 2).min(10_000);
                        }
                    }
                }
            };

            for bucket in response.accepted_transaction_ids.iter() {
                let found = bucket
                    .accepted_transaction_ids
                    .iter()
                    .any(|id| id.to_string() == tx_id_str);
                if found {
                    return Ok(Some(bucket.accepting_block_hash.to_string()));
                }
            }

            let added = &response.added_chain_block_hashes;
            let Some(last) = added.last().copied() else { break; };
            if last == start_hash {
                break;
            }
            start_hash = last;
        }

        Ok(None)
    }

    pub async fn get_network_info(&self) -> Result<String> {
        let info = self
            .client
            .get_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        Ok(format!(
            "Server version: {}\nMempool size: {}\nSynced: {}\nP2P ID: {}\nUTXO Indexed: {}",
            info.server_version,
            info.mempool_size,
            info.is_synced,
            info.p2p_id,
            info.is_utxo_indexed
        ))
    }
}