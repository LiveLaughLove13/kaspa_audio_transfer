//! Kaspa network client for file transfer and wallet operations.

mod estimate;
mod kat;
mod receive;
mod rest;
mod send;
mod tx_hash;
mod wallet;

use kaspa_consensus_core::constants::SOMPI_PER_KASPA;
use kaspa_rpc_core::api::rpc::RpcApi;

use crate::error::{AudioTransferError, Result};
use kaspa_consensus_core::network::{NetworkId, NetworkType};
use std::io::Write;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[cfg(feature = "grpc")]
use kaspa_grpc_client::GrpcClient;
#[cfg(feature = "grpc")]
use kaspa_rpc_core::notify::mode::NotificationMode;
use kaspa_wrpc_client::client::ConnectOptions;
use kaspa_wrpc_client::{KaspaRpcClient, Resolver, WrpcEncoding};

pub use kat::{FileId, KatManifest, MAX_CHUNK_DATA_SIZE};
pub use rest::rest_string_or_vec_to_vec;

const DEFAULT_KASPA_RPC_URL: &str = "grpc://127.0.0.1:16110";
pub(crate) const MAX_STANDARD_COMPUTE_MASS: u64 = 100_000;
pub(crate) const MAX_STANDARD_TRANSIENT_MASS: u64 = 100_000;
pub(crate) const MAX_STANDARD_STORAGE_MASS: u64 = 100_000;
pub(crate) const VIRTUAL_CHAIN_PAGE_LIMIT: u64 = 250;
pub(crate) const PUBLIC_VIRTUAL_CHAIN_PAGE_LIMIT_ANCHORED: u64 = 100;

pub struct KaspaClient {
    pub(crate) rpc: Arc<dyn RpcApi + Send + Sync>,
    pub(crate) is_public_resolver: bool,
    pub(crate) resolver_network_id: Option<NetworkId>,
}

impl KaspaClient {
    pub(crate) const MAX_RPC_SCAN_PAGES_FROM_PRUNING: u32 = 250;
    pub(crate) const RESOLVER_MAX_UTXO_INDEX_ATTEMPTS: u32 = 12;
    pub(crate) const RESOLVER_RECONNECT_ATTEMPTS: u32 = 5;

    pub async fn new(rpc_url: Option<&str>) -> Result<Self> {
        let rpc_url = rpc_url.unwrap_or(DEFAULT_KASPA_RPC_URL).trim();

        if let Some(network_id) = Self::parse_public_resolver_spec(rpc_url) {
            eprintln!(
                "Connecting to Kaspa public node network via resolver (wRPC) on network: {}",
                network_id
            );
            let wrpc = Self::connect_wrpc_via_resolver(network_id).await?;
            eprintln!("Successfully connected via public resolver (wRPC)");
            return Ok(Self {
                rpc: wrpc,
                is_public_resolver: true,
                resolver_network_id: Some(network_id),
            });
        }

        eprintln!("Connecting to Kaspa node at: {}", rpc_url);

        #[cfg(not(feature = "grpc"))]
        {
            return Err(AudioTransferError::KaspaRpc(
                "gRPC RPC is disabled in this build (grpc feature off). Use public resolver mode (public/public:tn10)".to_string(),
            ));
        }

        #[cfg(feature = "grpc")]
        {
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

            let rpc: Arc<dyn RpcApi + Send + Sync> = Arc::new(client);
            rpc.get_info().await.map_err(|e| {
                AudioTransferError::KaspaRpc(format!("Failed to connect to Kaspa node: {e}"))
            })?;

            eprintln!("Successfully connected to Kaspa node");
            Ok(Self {
                rpc,
                is_public_resolver: false,
                resolver_network_id: None,
            })
        }
    }

    fn parse_public_resolver_spec(rpc_url: &str) -> Option<NetworkId> {
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
        if network.is_empty() || network == "mainnet" {
            return Some(NetworkId::new(NetworkType::Mainnet));
        }
        if network == "devnet" {
            return Some(NetworkId::new(NetworkType::Devnet));
        }
        if network == "testnet"
            || network == "tn10"
            || network == "testnet10"
            || network == "testnet-10"
        {
            return Some(NetworkId::with_suffix(NetworkType::Testnet, 10));
        }

        None
    }

    pub(crate) async fn connect_wrpc_via_resolver(
        network_id: NetworkId,
    ) -> Result<Arc<dyn RpcApi + Send + Sync>> {
        let mut last_err: Option<String> = None;

        for attempt in 1..=Self::RESOLVER_MAX_UTXO_INDEX_ATTEMPTS {
            let resolver = Resolver::default();
            let client = KaspaRpcClient::new(
                WrpcEncoding::Borsh,
                None,
                Some(resolver),
                Some(network_id),
                None,
            )
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

            let client = Arc::new(client);
            let opts = ConnectOptions {
                block_async_connect: true,
                ..Default::default()
            };
            if let Err(e) = client.connect(Some(opts)).await {
                last_err = Some(e.to_string());
                sleep(Duration::from_millis(250)).await;
                continue;
            }

            let rpc = client.rpc_api();
            match rpc.get_info().await {
                Ok(info) => {
                    if info.is_utxo_indexed && info.is_synced {
                        return Ok(rpc);
                    }
                    last_err = Some(format!(
                        "connected node not suitable (utxo_indexed={}, synced={})",
                        info.is_utxo_indexed, info.is_synced
                    ));
                }
                Err(e) => {
                    last_err = Some(e.to_string());
                }
            }

            if attempt < Self::RESOLVER_MAX_UTXO_INDEX_ATTEMPTS {
                sleep(Duration::from_millis(250)).await;
            }
        }

        Err(AudioTransferError::KaspaRpc(format!(
            "Unable to find a public resolver node that is UTXO-indexed and synced after {} attempt(s). Last error: {}",
            Self::RESOLVER_MAX_UTXO_INDEX_ATTEMPTS,
            last_err.unwrap_or_else(|| "unknown".to_string())
        )))
    }

    pub(crate) fn network_prefix(&self, network: NetworkId) -> kaspa_addresses::Prefix {
        network.network_type().into()
    }

    pub(crate) fn params_for_network(
        network: NetworkId,
    ) -> kaspa_consensus_core::config::params::Params {
        kaspa_consensus_core::config::params::Params::from(network)
    }

    pub(crate) fn parse_private_key_hex(hex_str: &str) -> Result<[u8; 32]> {
        let bytes = hex::decode(hex_str).map_err(|e| {
            AudioTransferError::InvalidInput(format!("Invalid private key hex: {e}"))
        })?;
        if bytes.len() != 32 {
            return Err(AudioTransferError::InvalidInput(
                "Private key must be 32 bytes (64 hex chars)".to_string(),
            ));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(out)
    }

    pub(crate) fn sompi_to_kas(v: u64) -> f64 {
        v as f64 / SOMPI_PER_KASPA as f64
    }

    pub(crate) fn progress_line(line: &str) {
        eprint!("\r{:<140}", line);
        let _ = std::io::stderr().flush();
    }

    pub(crate) fn progress_end() {
        eprintln!();
    }

    pub(crate) async fn feerate_priority(&self) -> Result<f64> {
        let est = self
            .rpc
            .get_fee_estimate()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        Ok(est.priority_bucket.feerate)
    }

    pub(crate) async fn resolve_feerate(
        &self,
        feerate: Option<f64>,
        fee_multiplier: Option<f64>,
    ) -> Result<f64> {
        let est = self
            .rpc
            .get_fee_estimate()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;
        let min_feerate = est.low_buckets.first().map(|b| b.feerate).unwrap_or(0.0);
        let priority = est.priority_bucket.feerate;

        let mut effective = if let Some(v) = feerate {
            v
        } else if let Some(m) = fee_multiplier {
            priority * m
        } else {
            priority
        };

        if !effective.is_finite() || effective <= 0.0 {
            return Err(AudioTransferError::InvalidInput(
                "feerate must be a finite number > 0 (sompi/gram)".to_string(),
            ));
        }
        if min_feerate.is_finite() && min_feerate > 0.0 {
            effective = effective.max(min_feerate);
        }

        Ok(effective)
    }

    pub(crate) fn should_use_rest_fallback(err: &AudioTransferError) -> bool {
        match err {
            AudioTransferError::KaspaRpc(msg) => {
                let m = msg.to_lowercase();
                m.contains("transaction not found in mempool or virtual chain scan")
                    || m.contains("cannot find header")
                    || m.contains("header not found")
                    || m.contains("websocket disconnected")
                    || (m.contains("websocket") && m.contains("disconnect"))
                    || m.contains("connection closed")
                    || m.contains("broken pipe")
                    || m.contains("unexpected eof")
            }
            _ => false,
        }
    }

    pub(crate) fn is_resolver_disconnect_error(err: &AudioTransferError) -> bool {
        match err {
            AudioTransferError::KaspaRpc(msg) => {
                let m = msg.to_lowercase();
                m.contains("websocket disconnected")
                    || (m.contains("websocket") && m.contains("disconnect"))
                    || m.contains("connection closed")
                    || m.contains("broken pipe")
                    || m.contains("unexpected eof")
            }
            _ => false,
        }
    }

    pub async fn get_network_info(&self) -> Result<String> {
        let info = self
            .rpc
            .get_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        let dag = self
            .rpc
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        Ok(format!(
            "Network: {}\nServer version: {}\nMempool size: {}\nSynced: {}\nP2P ID: {}\nUTXO Indexed: {}\nPruning point: {}",
            dag.network,
            info.server_version,
            info.mempool_size,
            info.is_synced,
            info.p2p_id,
            info.is_utxo_indexed,
            dag.pruning_point_hash
        ))
    }

    pub async fn wallet_network_name(&self) -> Result<String> {
        let dag = self
            .rpc
            .get_block_dag_info()
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

        let network = match dag.network.network_type() {
            NetworkType::Mainnet => "mainnet",
            NetworkType::Testnet => "testnet",
            NetworkType::Devnet => "devnet",
            other => {
                return Err(AudioTransferError::InvalidInput(format!(
                    "Unsupported network type for wallet derivation: {other}"
                )))
            }
        };

        Ok(network.to_string())
    }

    pub(crate) fn is_missing_header_error(msg: &str) -> bool {
        let m = msg.to_lowercase();
        m.contains("cannot find header") || m.contains("header not found")
    }
}
