//! Fee estimation for audio transfer.

use super::kat::{self, KatManifest, MAX_CHUNK_DATA_SIZE};
use super::KaspaClient;
use crate::error::{AudioTransferError, Result};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::constants::{MAX_TX_IN_SEQUENCE_NUM, SOMPI_PER_KASPA, TX_VERSION};
use kaspa_consensus_core::mass::MassCalculator;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    Transaction, TransactionId, TransactionInput, TransactionOutpoint, TransactionOutput,
};
use kaspa_txscript::pay_to_address_script;
use rand::RngCore;
use secp256k1::Keypair;

impl KaspaClient {
    pub async fn estimate_audio_fees(
        &self,
        audio_data: &[u8],
        from_private_key_hex: &str,
        amount: f64,
    ) -> Result<()> {
        let send_value = (amount * SOMPI_PER_KASPA as f64) as u64;
        if send_value == 0 {
            return Err(AudioTransferError::InvalidInput(
                "amount must be > 0 (in KAS)".to_string(),
            ));
        }

        let dag = self
            .rpc
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
            .rpc
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
            vec![TransactionOutput::new(
                initial_entry_amount.max(1),
                self_spk.clone(),
            )],
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
        let total_chunks = audio_data.len().div_ceil(chunk_data_size) as u32;

        println!(
            "Chunking file into {} chunk(s) of up to {} bytes each",
            total_chunks, chunk_data_size
        );

        let mut file_id: kat::FileId = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut file_id);
        let manifest = KatManifest {
            file_id,
            total_size: audio_data.len() as u64,
            chunk_size: chunk_data_size as u32,
            total_chunks,
        };
        let manifest_payload = kat::encode_manifest_payload(&manifest);

        let feerate = self.feerate_priority().await?;
        let worst_chunk_payload_len = (4 + 1 + 16 + 4 + 4 + 4) + chunk_data_size;

        let mut assumed_input_amount = initial_entry_amount.max(1);
        let mut manifest_fee: u64 = 0;
        let mut chunk_fees: u64 = 0;

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
                    if msg
                        .to_lowercase()
                        .contains("insufficient funds for chunk fees") =>
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
                "Info: your largest available UTXO ({:.8} KAS) appears insufficient for amount+fees; estimate assumes {:.8} KAS input",
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
}
