//! Wallet operations: balance, address, send KAS.

use super::{
    KaspaClient, MAX_STANDARD_COMPUTE_MASS, MAX_STANDARD_STORAGE_MASS, MAX_STANDARD_TRANSIENT_MASS,
};
use crate::error::{AudioTransferError, Result};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::constants::{
    MAX_TX_IN_SEQUENCE_NUM, SOMPI_PER_KASPA, TX_VERSION, UNACCEPTED_DAA_SCORE,
};
use kaspa_consensus_core::mass::MassCalculator;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    PopulatedTransaction, Transaction, TransactionInput, TransactionOutput, UtxoEntry,
};
use kaspa_rpc_core::model::RpcTransactionId;
use kaspa_txscript::pay_to_address_script;
use secp256k1::Keypair;

impl KaspaClient {
    pub async fn wallet_balance_kas(&self, from_private_key_hex: &str) -> Result<f64> {
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

        let utxos = self
            .rpc
            .get_utxos_by_addresses(vec![from_addr])
            .await
            .map_err(|e| {
                AudioTransferError::KaspaRpc(format!(
                    "get_utxos_by_addresses failed (node needs --utxoindex): {e}"
                ))
            })?;
        let total_sompi: u64 = utxos.into_iter().map(|u| u.utxo_entry.amount).sum();
        Ok(Self::sompi_to_kas(total_sompi))
    }

    pub async fn wallet_address(&self, from_private_key_hex: &str) -> Result<String> {
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
        Ok(from_addr.to_string())
    }

    pub async fn wallet_send_kas_signed(
        &self,
        from_private_key_hex: &str,
        to_address: &str,
        amount: f64,
        feerate: Option<f64>,
        fee_multiplier: Option<f64>,
    ) -> Result<String> {
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
            self.resolve_initial_utxo(&from_addr, None, 1).await?;
        let effective_feerate = self.resolve_feerate(feerate, fee_multiplier).await?;

        let (tx_fee, change) = Self::estimate_manifest_fee_and_change(
            &mass_calc,
            effective_feerate,
            initial_entry_amount,
            send_value,
            &self_spk,
            &to_spk,
            vec![],
        )?;
        if change == 0 {
            return Err(AudioTransferError::KaspaRpc(
                "Insufficient funds for transfer + fee".to_string(),
            ));
        }

        let mut tx = Transaction::new(
            TX_VERSION,
            vec![TransactionInput::new(
                initial_outpoint,
                Vec::new(),
                MAX_TX_IN_SEQUENCE_NUM,
                1,
            )],
            vec![
                TransactionOutput::new(send_value, to_spk),
                TransactionOutput::new(change.max(1), self_spk),
            ],
            0,
            SUBNETWORK_ID_NATIVE,
            0,
            vec![],
        );

        tx.inputs[0].signature_script = vec![0u8; 66];
        tx.finalize();

        let populated = PopulatedTransaction::new(
            &tx,
            vec![UtxoEntry::new(
                initial_entry_amount,
                initial_entry_spk.clone(),
                UNACCEPTED_DAA_SCORE,
                false,
            )],
        );
        let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
        if non_ctx.compute_mass > MAX_STANDARD_COMPUTE_MASS
            || non_ctx.transient_mass > MAX_STANDARD_TRANSIENT_MASS
        {
            return Err(AudioTransferError::KaspaRpc(format!(
                "Transaction not standard (compute_mass={}, transient_mass={})",
                non_ctx.compute_mass, non_ctx.transient_mass
            )));
        }
        let ctx = mass_calc
            .calc_contextual_masses(&populated)
            .ok_or_else(|| AudioTransferError::KaspaRpc("Mass incomputable".to_string()))?;
        if ctx.storage_mass > MAX_STANDARD_STORAGE_MASS {
            return Err(AudioTransferError::KaspaRpc(format!(
                "Transaction not standard (storage_mass={} > {})",
                ctx.storage_mass, MAX_STANDARD_STORAGE_MASS
            )));
        }
        tx.set_mass(ctx.storage_mass);
        tx = Self::finalize_and_sign_single_input(
            tx,
            UtxoEntry::new(
                initial_entry_amount,
                initial_entry_spk,
                UNACCEPTED_DAA_SCORE,
                false,
            ),
            &keypair,
        )?;

        let submitted: RpcTransactionId = self
            .rpc
            .submit_transaction((&tx).into(), false)
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        println!("Transfer fee: {:.8} KAS", Self::sompi_to_kas(tx_fee));
        Ok(submitted.to_string())
    }
}
