use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::{
    config::params::Params,
    constants::{MAX_TX_IN_SEQUENCE_NUM, SOMPI_PER_KASPA, TX_VERSION, UNACCEPTED_DAA_SCORE},
    mass::MassCalculator,
    network::{NetworkId, NetworkType},
    sign,
    subnets::SUBNETWORK_ID_NATIVE,
    tx::{
        PopulatedTransaction, Transaction, TransactionId, TransactionInput, TransactionOutpoint, TransactionOutput,
        UtxoEntry,
    },
};
use kaspa_rpc_core::{api::rpc::RpcApi, model::RpcTransactionId};
use kaspa_txscript::pay_to_address_script;
use kaspa_wrpc_client::client::ConnectOptions;
use kaspa_wrpc_client::{KaspaRpcClient, Resolver, WrpcEncoding};
use once_cell::sync::Lazy;
use secp256k1::Keypair;
use serde::Serialize;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::wallet_vault;

type DynRpc = Arc<dyn RpcApi + Send + Sync>;
const RESOLVER_MAX_UTXO_INDEX_ATTEMPTS: u32 = 12;

fn network_type_key(network: NetworkType) -> &'static str {
    match network {
        NetworkType::Mainnet => "mainnet",
        NetworkType::Testnet => "testnet",
        NetworkType::Devnet => "devnet",
        _ => "other",
    }
}

fn cache_key(rpc_url: &str, expected_network: NetworkType) -> String {
    format!("{}::{}", network_type_key(expected_network), rpc_url)
}

static RPC_CLIENT_CACHE: Lazy<Mutex<HashMap<String, DynRpc>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConnectionInfo {
    pub rpc_url: String,
    pub network: String,
    pub server_version: String,
    pub is_synced: bool,
    pub p2p_id: String,
    pub is_utxo_indexed: bool,
}

fn parse_network(network: &str) -> Result<NetworkType, String> {
    match network.trim() {
        "mainnet" => Ok(NetworkType::Mainnet),
        "testnet" => Ok(NetworkType::Testnet),
        "devnet" => Ok(NetworkType::Devnet),
        _ => Err("network must be one of: mainnet, testnet, devnet".to_string()),
    }
}

fn prefix_for_network(network: NetworkType) -> Prefix {
    network.into()
}

fn prefix_for_network_id(network_id: NetworkId) -> Prefix {
    network_id.network_type().into()
}

fn network_id_for_wallet_network(network: NetworkType) -> NetworkId {
    match network {
        NetworkType::Mainnet => NetworkId::new(NetworkType::Mainnet),
        NetworkType::Devnet => NetworkId::new(NetworkType::Devnet),
        NetworkType::Testnet => NetworkId::with_suffix(NetworkType::Testnet, 10),
        _ => NetworkId::new(network),
    }
}

fn parse_public_resolver_spec(rpc_url: &str, default_network: NetworkType) -> Option<NetworkId> {
    let raw = rpc_url.trim();
    if raw.is_empty() {
        return None;
    }
    let lower = raw.to_ascii_lowercase();
    let (head, tail) = lower.split_once(':').unwrap_or((&lower, ""));
    if head != "public" && head != "resolver" {
        return None;
    }

    let network = tail.trim();
    if network.is_empty() {
        return Some(network_id_for_wallet_network(default_network));
    }

    if network == "mainnet" {
        return Some(NetworkId::new(NetworkType::Mainnet));
    }
    if network == "devnet" {
        return Some(NetworkId::new(NetworkType::Devnet));
    }
    if network == "testnet" || network == "tn10" || network == "testnet10" || network == "testnet-10" {
        return Some(NetworkId::with_suffix(NetworkType::Testnet, 10));
    }

    None
}

async fn connect_client(rpc_url: Option<&str>, expected_network: NetworkType) -> Result<DynRpc, String> {
    let rpc_url = rpc_url
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("public");

    let key = cache_key(rpc_url, expected_network);

    {
        let guard = RPC_CLIENT_CACHE.lock().await;
        if let Some(cached) = guard.get(&key) {
            return Ok(cached.clone());
        }
    }

    let Some(network_id) = parse_public_resolver_spec(rpc_url, expected_network) else {
        return Err("mobile wallet supports public resolver only (use 'public' or 'public:tn10')".to_string());
    };

    let mut last_err: Option<String> = None;
    for attempt in 1..=RESOLVER_MAX_UTXO_INDEX_ATTEMPTS {
        let resolver = Resolver::default();
        let client = KaspaRpcClient::new(
            WrpcEncoding::Borsh,
            None,
            Some(resolver),
            Some(network_id),
            None,
        )
        .map_err(|e| format!("failed to create kaspa wRPC client: {e}"))?;

        let client = Arc::new(client);
        let mut opts = ConnectOptions::default();
        opts.block_async_connect = true;
        if let Err(e) = client.connect(Some(opts)).await {
            last_err = Some(e.to_string());
            sleep(Duration::from_millis(250)).await;
            continue;
        }

        let rpc = client.rpc_api();
        match rpc.get_info().await {
            Ok(info) => {
                if info.is_utxo_indexed {
                    {
                        let mut guard = RPC_CLIENT_CACHE.lock().await;
                        guard.insert(key.clone(), rpc.clone());
                    }
                    return Ok(rpc);
                }
                last_err = Some("connected node is not UTXO-indexed".to_string());
            }
            Err(e) => last_err = Some(e.to_string()),
        }

        if attempt < RESOLVER_MAX_UTXO_INDEX_ATTEMPTS {
            sleep(Duration::from_millis(250)).await;
        }
    }

    Err(format!(
        "unable to find a public resolver node with UTXO index enabled after {} attempt(s). last error: {}",
        RESOLVER_MAX_UTXO_INDEX_ATTEMPTS,
        last_err.unwrap_or_else(|| "unknown".to_string())
    ))
}

pub async fn rpc_connection_info(network: &str, rpc_url: Option<&str>) -> Result<RpcConnectionInfo, String> {
    let expected = parse_network(network)?;
    let rpc_url_norm = rpc_url
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("public");

    let client = connect_client(Some(rpc_url_norm), expected).await?;
    let info = client.get_info().await.map_err(|e| e.to_string())?;
    let dag = client.get_block_dag_info().await.map_err(|e| e.to_string())?;

    Ok(RpcConnectionInfo {
        rpc_url: rpc_url_norm.to_string(),
        network: format!("{}", dag.network),
        server_version: info.server_version,
        is_synced: info.is_synced,
        p2p_id: info.p2p_id,
        is_utxo_indexed: info.is_utxo_indexed,
    })
}

fn address_from_keypair(prefix: Prefix, keypair: &Keypair) -> Address {
    let xonly_pk = keypair.public_key().x_only_public_key().0;
    Address::new(prefix, Version::PubKey, &xonly_pk.serialize())
}

async fn get_mempool_spent_outpoints(client: &DynRpc, from_addr: &Address) -> HashSet<TransactionOutpoint> {
    let mut spent: HashSet<TransactionOutpoint> = HashSet::new();

    if let Ok(mut by_addr) = client
        .get_mempool_entries_by_addresses(vec![from_addr.clone()], true, false)
        .await
    {
        if let Some(entry) = by_addr.pop() {
            for e in entry.sending {
                for i in e.transaction.inputs {
                    spent.insert(TransactionOutpoint::new(
                        i.previous_outpoint.transaction_id,
                        i.previous_outpoint.index,
                    ));
                }
            }
        }
    }

    spent
}

async fn pick_largest_utxo(
    client: &DynRpc,
    from_addr: &Address,
) -> Result<(TransactionOutpoint, u64, kaspa_consensus_core::tx::ScriptPublicKey), String> {
    let mempool_spent = get_mempool_spent_outpoints(client, from_addr).await;

    let mut utxos = client
        .get_utxos_by_addresses(vec![from_addr.clone()])
        .await
        .map_err(|e| format!("get_utxos_by_addresses failed (node needs UTXO index): {e}"))?;

    if !mempool_spent.is_empty() {
        utxos.retain(|u| {
            let o = TransactionOutpoint::new(u.outpoint.transaction_id, u.outpoint.index);
            !mempool_spent.contains(&o)
        });
    }

    if utxos.is_empty() {
        return Err("No spendable UTXOs found for address".to_string());
    }

    utxos.sort_by_key(|u| std::cmp::Reverse(u.utxo_entry.amount));
    let picked = utxos.into_iter().next().unwrap();
    Ok((
        TransactionOutpoint::new(picked.outpoint.transaction_id, picked.outpoint.index),
        picked.utxo_entry.amount,
        picked.utxo_entry.script_public_key,
    ))
}

fn estimate_fee_and_change(
    mass_calc: &MassCalculator,
    feerate_sompi_per_gram: f64,
    input_amount: u64,
    input_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
    to_spk: &kaspa_consensus_core::tx::ScriptPublicKey,
    send_value: u64,
) -> Result<(u64, u64, u64), String> {
    if send_value == 0 {
        return Err("amount must be > 0".to_string());
    }
    if input_amount == 0 {
        return Err("no input funds".to_string());
    }
    if !feerate_sompi_per_gram.is_finite() || feerate_sompi_per_gram <= 0.0 {
        return Err("invalid feerate".to_string());
    }

    let mut change_guess: u64 = input_amount.saturating_sub(send_value).max(1);
    let mut last_fee: u64 = 0;

    for _ in 0..20u32 {
        let outputs = vec![
            TransactionOutput::new(send_value, to_spk.clone()),
            TransactionOutput::new(change_guess, input_spk.clone()),
        ];

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
            vec![],
        );
        tx.finalize();

        let entry = UtxoEntry::new(input_amount, input_spk.clone(), UNACCEPTED_DAA_SCORE, false);
        let populated = PopulatedTransaction::new(&tx, vec![entry]);

        let non_ctx = mass_calc.calc_non_contextual_masses(&tx);
        let ctx = mass_calc
            .calc_contextual_masses(&populated)
            .ok_or_else(|| "Mass incomputable".to_string())?;

        let total_mass = non_ctx
            .compute_mass
            .max(non_ctx.transient_mass)
            .max(ctx.storage_mass);

        let fee = (total_mass as f64 * feerate_sompi_per_gram).ceil() as u64;
        let required = send_value.saturating_add(fee);
        let new_change = input_amount.saturating_sub(required);

        if new_change == 0 {
            return Ok((fee, 0, ctx.storage_mass));
        }

        if new_change == change_guess && fee == last_fee {
            return Ok((fee, new_change, ctx.storage_mass));
        }

        last_fee = fee;
        change_guess = new_change;
    }

    let required = send_value.saturating_add(last_fee);
    let final_change = input_amount.saturating_sub(required);
    Ok((last_fee, final_change, 0))
}

fn sign_single_input(tx: Transaction, entry: UtxoEntry, keypair: &Keypair) -> Result<Transaction, String> {
    if tx.inputs.is_empty() {
        return Err("transaction must have at least one input".to_string());
    }

    let mut tx = tx;
    tx.inputs[0].sig_op_count = 1;

    let mut signable = kaspa_consensus_core::tx::SignableTransaction::new(tx);
    signable.entries[0] = Some(entry);

    let mut signed = sign::sign(signable, keypair.clone());
    signed.tx.finalize();
    Ok(signed.tx)
}

pub async fn wallet_get_balance_kas(
    network: &str,
    derivation_path: &str,
    rpc_url: Option<&str>,
) -> Result<f64, String> {
    let expected = parse_network(network)?;

    let client = connect_client(rpc_url, expected).await?;
    let dag = client
        .get_block_dag_info()
        .await
        .map_err(|e| format!("get_block_dag_info failed: {e}"))?;

    let prefix = prefix_for_network_id(dag.network);
    let expected_prefix = prefix_for_network(expected);
    if expected_prefix != prefix {
        return Err(format!(
            "network mismatch: node prefix is '{prefix}', but UI selected '{expected_prefix}'"
        ));
    }

    let keypair = wallet_vault::derive_keypair_for_path(network, derivation_path)?;
    let from_addr = address_from_keypair(prefix, &keypair);

    let utxos = client
        .get_utxos_by_addresses(vec![from_addr])
        .await
        .map_err(|e| format!("get_utxos_by_addresses failed (node needs UTXO index): {e}"))?;

    let total_sompi: u64 = utxos.into_iter().map(|u| u.utxo_entry.amount).sum();
    Ok(total_sompi as f64 / SOMPI_PER_KASPA as f64)
}

pub async fn wallet_send_kas(
    network: &str,
    derivation_path: &str,
    rpc_url: Option<&str>,
    to_address: &str,
    amount_kas: f64,
) -> Result<String, String> {
    let expected = parse_network(network)?;

    if !amount_kas.is_finite() || amount_kas <= 0.0 {
        return Err("amount must be a finite number > 0".to_string());
    }

    let send_value = (amount_kas * SOMPI_PER_KASPA as f64) as u64;
    if send_value == 0 {
        return Err("amount too small".to_string());
    }

    let client = connect_client(rpc_url, expected).await?;
    let dag = client
        .get_block_dag_info()
        .await
        .map_err(|e| format!("get_block_dag_info failed: {e}"))?;

    let prefix = prefix_for_network_id(dag.network);
    let expected_prefix = prefix_for_network(expected);
    if expected_prefix != prefix {
        return Err(format!(
            "network mismatch: node prefix is '{prefix}', but UI selected '{expected_prefix}'"
        ));
    }

    let to_addr = Address::try_from(to_address.trim()).map_err(|e| format!("invalid to_address: {e}"))?;
    if to_addr.prefix != prefix {
        return Err(format!(
            "Address prefix mismatch. Node network prefix is '{prefix}', but to_address is '{}'",
            to_addr.prefix
        ));
    }

    let keypair = wallet_vault::derive_keypair_for_path(network, derivation_path)?;
    let from_addr = address_from_keypair(prefix, &keypair);

    let (outpoint, input_amount, input_spk) = pick_largest_utxo(&client, &from_addr).await?;

    let params = Params::from(dag.network);
    let mass_calc = MassCalculator::new(
        params.mass_per_tx_byte,
        params.mass_per_script_pub_key_byte,
        params.mass_per_sig_op,
        params.storage_mass_parameter,
    );

    let fee_est = client
        .get_fee_estimate()
        .await
        .map_err(|e| format!("get_fee_estimate failed: {e}"))?;

    let feerate = fee_est.priority_bucket.feerate;

    let self_spk = pay_to_address_script(&from_addr);
    let to_spk = pay_to_address_script(&to_addr);

    let (fee, change, storage_mass) = estimate_fee_and_change(
        &mass_calc,
        feerate,
        input_amount,
        &input_spk,
        &to_spk,
        send_value,
    )?;

    if change == 0 {
        return Err(format!(
            "insufficient funds: input {:.8} KAS is not enough for amount+fee (fee approx {:.8} KAS)",
            input_amount as f64 / SOMPI_PER_KASPA as f64,
            fee as f64 / SOMPI_PER_KASPA as f64
        ));
    }

    let outputs = vec![
        TransactionOutput::new(send_value, to_spk),
        TransactionOutput::new(change.max(1), self_spk.clone()),
    ];

    let mut tx = Transaction::new(
        TX_VERSION,
        vec![TransactionInput::new(outpoint, Vec::new(), MAX_TX_IN_SEQUENCE_NUM, 1)],
        outputs,
        0,
        SUBNETWORK_ID_NATIVE,
        0,
        vec![],
    );

    tx.inputs[0].signature_script = vec![0u8; 66];
    tx.finalize();

    let entry = UtxoEntry::new(input_amount, input_spk, UNACCEPTED_DAA_SCORE, false);
    let populated = PopulatedTransaction::new(&tx, vec![entry.clone()]);
    let ctx = mass_calc
        .calc_contextual_masses(&populated)
        .ok_or_else(|| "Mass incomputable".to_string())?;

    let mass_to_set = if ctx.storage_mass > 0 { ctx.storage_mass } else { storage_mass };
    if mass_to_set > 0 {
        tx.set_mass(mass_to_set);
    }

    tx = sign_single_input(tx, entry, &keypair)?;

    let submitted: RpcTransactionId = client
        .submit_transaction((&tx).into(), false)
        .await
        .map_err(|e| format!("submit_transaction failed: {e}"))?;

    Ok(submitted.to_string())
}
