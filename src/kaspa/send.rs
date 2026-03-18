//! Kaspa send operations: audio transfer and transaction signing.

use super::kat::{self, KatManifest, MAX_CHUNK_DATA_SIZE};
use super::{
    KaspaClient, MAX_STANDARD_COMPUTE_MASS, MAX_STANDARD_STORAGE_MASS, MAX_STANDARD_TRANSIENT_MASS,
};
use crate::error::{AudioTransferError, Result};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::constants::{
    MAX_TX_IN_SEQUENCE_NUM, SOMPI_PER_KASPA, TX_VERSION, UNACCEPTED_DAA_SCORE,
};
use kaspa_consensus_core::mass::MassCalculator;
use kaspa_consensus_core::sign;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    PopulatedTransaction, Transaction, TransactionId, TransactionInput, TransactionOutpoint,
    TransactionOutput, UtxoEntry,
};
use kaspa_rpc_core::model::RpcTransactionId;
use kaspa_txscript::pay_to_address_script;
use rand::RngCore;
use secp256k1::Keypair;
use std::collections::HashSet;
use std::str::FromStr;

impl KaspaClient {
    #[allow(clippy::too_many_arguments)]
    pub async fn send_audio_signed(
        &self,
        audio_data: &[u8],
        from_private_key_hex: &str,
        to_address: &str,
        amount: f64,
        resume_from: Option<&str>,
        resume_output_index: u32,
        feerate: Option<f64>,
        fee_multiplier: Option<f64>,
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
            .rpc
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

        let (initial_outpoint, initial_entry_amount, initial_entry_spk) = self
            .resolve_initial_utxo(&from_addr, resume_from, resume_output_index)
            .await?;

        let chunk_overhead = 4 + 1 + 16 + 4 + 4 + 4;
        let chunk_template = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(
                initial_outpoint,
                Vec::new(),
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
            initial_entry_spk.clone(),
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

        let feerate = self.resolve_feerate(feerate, fee_multiplier).await?;
        let (manifest_fee, manifest_change) = Self::estimate_manifest_fee_and_change(
            &mass_calc,
            feerate,
            initial_entry_amount,
            send_value,
            &self_spk,
            &to_spk,
            kat::encode_manifest_payload(&manifest),
        )?;
        if manifest_change == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Insufficient funds for manifest tx".to_string(),
            ));
        }

        let mut outputs = Vec::new();
        if send_value > 0 {
            outputs.push(TransactionOutput::new(send_value, to_spk.clone()));
        }
        outputs.push(TransactionOutput::new(
            manifest_change.max(1),
            self_spk.clone(),
        ));

        let mut manifest_tx = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(
                initial_outpoint,
                Vec::new(),
                MAX_TX_IN_SEQUENCE_NUM,
                1,
            )],
            outputs,
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            kat::encode_manifest_payload(&manifest),
        );
        manifest_tx.inputs[0].signature_script = vec![0u8; 66];
        manifest_tx.finalize();

        let populated = PopulatedTransaction::new(
            &manifest_tx,
            vec![UtxoEntry::new(
                initial_entry_amount,
                initial_entry_spk.clone(),
                UNACCEPTED_DAA_SCORE,
                false,
            )],
        );
        let non_ctx = mass_calc.calc_non_contextual_masses(&manifest_tx);
        if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS
            || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS
        {
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
            UtxoEntry::new(
                initial_entry_amount,
                initial_entry_spk.clone(),
                UNACCEPTED_DAA_SCORE,
                false,
            ),
            &keypair,
        )?;

        let manifest_txid: RpcTransactionId = self
            .rpc
            .submit_transaction((&manifest_tx).into(), false)
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        println!("Submitted manifest tx: {manifest_txid}");

        let change_index = if send_value > 0 { 1 } else { 0 };
        let mut next_outpoint = TransactionOutpoint::new(manifest_tx.id(), change_index);
        let mut next_entry = UtxoEntry::new(
            manifest_change,
            self_spk.clone(),
            UNACCEPTED_DAA_SCORE,
            false,
        );
        let mut chain_value = manifest_change;
        let mut total_chunk_fees: u64 = 0;

        for chunk_index in 0..total_chunks {
            let start = (chunk_index as usize) * chunk_data_size;
            let end = (start + chunk_data_size).min(audio_data.len());
            let data = &audio_data[start..end];
            let payload =
                kat::encode_chunk_payload(&manifest.file_id, chunk_index, total_chunks, data);

            let mut tx = Transaction::new(
                TX_VERSION,
                vec![TransactionInput::new(
                    next_outpoint,
                    Vec::new(),
                    MAX_TX_IN_SEQUENCE_NUM,
                    1,
                )],
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
                return Err(AudioTransferError::KaspaRpc(
                    "Insufficient funds for chunk fee".to_string(),
                ));
            }

            tx.outputs[0].value = out_value;

            tx.inputs[0].signature_script = vec![0u8; 66];
            tx.finalize();

            let populated = PopulatedTransaction::new(&tx, vec![next_entry.clone()]);
            let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
            if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS
                || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS
            {
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
            let tx = Self::finalize_and_sign_single_input(tx, next_entry.clone(), &keypair)?;

            let submitted: RpcTransactionId = self
                .rpc
                .submit_transaction((&tx).into(), false)
                .await
                .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

            next_outpoint = TransactionOutpoint::new(tx.id(), 0);
            chain_value = out_value;
            next_entry = UtxoEntry::new(out_value, self_spk.clone(), UNACCEPTED_DAA_SCORE, false);

            if (chunk_index + 1) % 10 == 0 || chunk_index + 1 == total_chunks {
                println!(
                    "Submitted chunk {}/{}: {}",
                    chunk_index + 1,
                    total_chunks,
                    submitted
                );
            }
        }

        let total_fee_sompi = total_chunk_fees.saturating_add(manifest_fee);
        println!(
            "Fee summary: manifest {:.8} KAS, chunks {:.8} KAS, total {:.8} KAS",
            super::KaspaClient::sompi_to_kas(manifest_fee),
            super::KaspaClient::sompi_to_kas(total_chunk_fees),
            super::KaspaClient::sompi_to_kas(total_fee_sompi)
        );

        Ok(manifest_txid.to_string())
    }

    pub(crate) async fn resolve_initial_utxo(
        &self,
        from_addr: &Address,
        resume_from: Option<&str>,
        resume_output_index: u32,
    ) -> Result<(
        TransactionOutpoint,
        u64,
        kaspa_consensus_core::tx::ScriptPublicKey,
    )> {
        if let Some(resume_txid_str) = resume_from {
            let resume_txid = TransactionId::from_str(resume_txid_str).map_err(|e| {
                AudioTransferError::InvalidInput(format!("Invalid resume_from txid: {e}"))
            })?;
            let outpoint = TransactionOutpoint::new(resume_txid, resume_output_index);
            let idx = resume_output_index as usize;

            let mempool_out = self
                .rpc
                .get_mempool_entry(resume_txid, true, false)
                .await
                .ok()
                .and_then(|e| e.transaction.outputs.get(idx).cloned());

            if let Some(out) = mempool_out {
                return Ok((outpoint, out.value, out.script_public_key));
            }

            let utxos = self
                .rpc
                .get_utxos_by_addresses(vec![from_addr.clone()])
                .await
                .map_err(|e| {
                    AudioTransferError::KaspaRpc(format!(
                        "get_utxos_by_addresses failed (node needs --utxoindex): {e}"
                    ))
                })?;

            let found = utxos
                .into_iter()
                .find(|u| {
                    u.outpoint.transaction_id == resume_txid
                        && u.outpoint.index == resume_output_index
                })
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
            .rpc
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

    pub(crate) fn compute_max_payload_len_for_template(
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
        if base_non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS
            || base_non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS
        {
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
            let mid = (lo + hi).div_ceil(2);
            tx.payload = vec![0u8; mid];
            tx.finalize();

            let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
            if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS
                || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS
            {
                hi = mid - 1;
                continue;
            }

            let entry =
                UtxoEntry::new(input_amount, input_spk.clone(), UNACCEPTED_DAA_SCORE, false);
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

    pub(crate) fn estimate_manifest_fee_and_change(
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

    pub(crate) fn estimate_total_chunk_fees(
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

    pub(crate) fn finalize_and_sign_single_input(
        mut tx: Transaction,
        entry: UtxoEntry,
        keypair: &Keypair,
    ) -> Result<Transaction> {
        if tx.inputs.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "Transaction must have at least one input".to_string(),
            ));
        }
        tx.inputs[0].sig_op_count = 1;
        let mut signable = kaspa_consensus_core::tx::SignableTransaction::new(tx);
        signable.entries[0] = Some(entry);

        let mut signed = sign::sign(signable, *keypair);
        signed.tx.finalize();
        Ok(signed.tx)
    }
}
