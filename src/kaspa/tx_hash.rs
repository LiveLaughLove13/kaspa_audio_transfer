//! Transaction accepting block hash resolution.

use kaspa_consensus_core::tx::TransactionId;
use kaspa_rpc_core::model::RpcHash;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

use super::KaspaClient;
use crate::error::{AudioTransferError, Result};

impl KaspaClient {
    pub async fn get_tx_accepting_block_hash(
        &self,
        tx_id: &str,
        start_block_hash: Option<&str>,
        min_confirmation_count: u64,
    ) -> Result<Option<String>> {
        let tx_id_str = tx_id;
        let tx_id = TransactionId::from_str(tx_id_str)
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid transaction ID: {e}")))?;

        if self.rpc.get_mempool_entry(tx_id, true, false).await.is_ok() {
            return Ok(None);
        }

        let mut effective_start_block_hash = start_block_hash;

        loop {
            let dag = self
                .rpc
                .get_block_dag_info()
                .await
                .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

            let mut start_hash: RpcHash = if let Some(h) = effective_start_block_hash {
                RpcHash::from_str(h).map_err(|e| {
                    AudioTransferError::KaspaRpc(format!("Invalid start_block_hash: {e}"))
                })?
            } else {
                dag.pruning_point_hash
            };

            let mut page: u32 = 0;
            let mut used_user_start_hash = effective_start_block_hash.is_some();
            'scan_accept: while page < 2000u32 {
                Self::progress_line(&format!(
                    "Scanning acceptance for tx {} | page {} | from {}{}",
                    tx_id,
                    page,
                    start_hash,
                    if used_user_start_hash {
                        " (user start)"
                    } else {
                        ""
                    }
                ));

                let mut attempt: u32 = 0;
                let mut backoff_ms: u64 = 250;
                let response = loop {
                    attempt += 1;
                    match self
                        .rpc
                        .get_virtual_chain_from_block(
                            start_hash,
                            true,
                            Some(min_confirmation_count),
                        )
                        .await
                    {
                        Ok(resp) => break resp,
                        Err(e) => {
                            let msg = e.to_string();
                            if used_user_start_hash && Self::is_missing_header_error(&msg) {
                                Self::progress_end();
                                eprintln!(
                                    "Info: start_block_hash header not found on this node; falling back to pruning point {}",
                                    dag.pruning_point_hash
                                );
                                eprintln!(
                                    "Hint: on pruned nodes, the explorer 'Accepting block hash' may be unavailable locally; try a tx 'Block hashes' value instead."
                                );
                                start_hash = dag.pruning_point_hash;
                                used_user_start_hash = false;
                                page = 0;
                                continue 'scan_accept;
                            }
                            let is_timeout = msg.to_lowercase().contains("timeout");
                            if !is_timeout || attempt >= 10 {
                                return Err(AudioTransferError::KaspaRpc(msg));
                            }
                            sleep(Duration::from_millis(backoff_ms)).await;
                            backoff_ms = (backoff_ms * 2).min(10_000);
                        }
                    }
                };

                for bucket in response.accepted_transaction_ids.iter() {
                    if bucket.accepted_transaction_ids.contains(&tx_id) {
                        Self::progress_end();
                        eprintln!(
                            "Accepting block hash (may be unavailable locally on pruned nodes): {}",
                            bucket.accepting_block_hash
                        );
                        eprintln!("Start block hash (scan anchor): {}", start_hash);
                        return Ok(Some(start_hash.to_string()));
                    }
                }

                let added = &response.added_chain_block_hashes;
                let Some(last) = added.last().copied() else {
                    break;
                };

                if last == start_hash {
                    break;
                }
                start_hash = last;
                page += 1;
            }

            if effective_start_block_hash.is_some() {
                Self::progress_end();
                eprintln!(
                    "Info: accepting block not found when scanning from start_block_hash; retrying from pruning point {}",
                    dag.pruning_point_hash
                );
                effective_start_block_hash = None;
                continue;
            }

            Self::progress_end();
            return Ok(None);
        }
    }
}
