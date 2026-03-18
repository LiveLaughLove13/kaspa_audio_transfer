//! Receive audio via RPC or REST fallback.

use kaspa_consensus_core::network::{NetworkId, NetworkType};
use kaspa_consensus_core::tx::TransactionId;
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_rpc_core::model::{RpcDataVerbosityLevel, RpcHash};
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use super::kat;
use super::rest;
use super::{KaspaClient, PUBLIC_VIRTUAL_CHAIN_PAGE_LIMIT_ANCHORED, VIRTUAL_CHAIN_PAGE_LIMIT};
use crate::error::{AudioTransferError, Result};

impl KaspaClient {
    pub(crate) async fn find_transaction_payload(
        &self,
        rpc: &Arc<dyn RpcApi + Send + Sync>,
        tx_id: TransactionId,
        start_block_hash: Option<&str>,
    ) -> Result<Vec<u8>> {
        if let Ok(entry) = rpc.get_mempool_entry(tx_id, true, false).await {
            return Ok(entry.transaction.payload);
        }

        let mut effective_start_block_hash = start_block_hash;

        loop {
            let dag = rpc
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

            let page_limit: u64 = if self.is_public_resolver {
                if effective_start_block_hash.is_some() {
                    PUBLIC_VIRTUAL_CHAIN_PAGE_LIMIT_ANCHORED
                } else {
                    25
                }
            } else {
                VIRTUAL_CHAIN_PAGE_LIMIT
            };
            let mut page: u32 = 0;
            let mut used_user_start_hash = effective_start_block_hash.is_some();
            'scan_tx: while page < 2000u32 {
                if !used_user_start_hash && page >= Self::MAX_RPC_SCAN_PAGES_FROM_PRUNING {
                    Self::progress_end();
                    return Err(AudioTransferError::KaspaRpc(
                        "Transaction not found in mempool or virtual chain scan (hit pruning-point scan limit). Hint: pass receive --start-block-hash with a scan-usable block hash (often from explorer 'Block hashes'), or run the tx-accepting-block-hash command to derive one from your node."
                            .to_string(),
                    ));
                }

                Self::progress_line(&format!(
                    "Scanning for tx {} | page {} | from {}{}",
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
                    match rpc
                        .get_virtual_chain_from_block_v2(
                            start_hash,
                            Some(RpcDataVerbosityLevel::Full),
                            Some(page_limit),
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
                                continue 'scan_tx;
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

                for block in response.chain_block_accepted_transactions.iter() {
                    for tx in block.accepted_transactions.iter() {
                        let Some(p) = tx.payload.as_ref() else {
                            continue;
                        };
                        let is_target = tx
                            .verbose_data
                            .as_ref()
                            .and_then(|v| v.transaction_id)
                            .is_some_and(|id| id == tx_id);
                        if !is_target {
                            continue;
                        }
                        Self::progress_end();
                        return Ok(p.clone());
                    }
                }

                let added = response.added_chain_block_hashes.as_ref();
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
                    "Info: transaction not found when scanning from start_block_hash; retrying from pruning point {}",
                    dag.pruning_point_hash
                );
                effective_start_block_hash = None;
                continue;
            }

            Self::progress_end();
            return Err(AudioTransferError::KaspaRpc(
                "Transaction not found in mempool or virtual chain scan".to_string(),
            ));
        }
    }

    pub(crate) async fn receive_audio_via_rest(
        &self,
        tx_id: &str,
        start_block_hash: Option<&str>,
    ) -> Result<Vec<u8>> {
        eprintln!("Info: falling back to api.kaspa.org for transaction/chunk retrieval");

        let tx = rest::rest_get_tx(tx_id, start_block_hash, false, true).await?;
        let payload = rest::rest_payload_hex_to_bytes(tx.payload)?;

        if !kat::is_kat_payload(&payload) {
            return Ok(payload);
        }

        if payload.len() >= 5 && payload[4] == kat::KAT_TYPE_CHUNK {
            let Some((_file_id, _idx, _total, offset)) = kat::try_decode_chunk_header(&payload)
            else {
                return Err(AudioTransferError::InvalidInput(
                    "Invalid chunk payload".to_string(),
                ));
            };
            if payload.len() < 33 {
                return Err(AudioTransferError::InvalidInput(
                    "Invalid chunk payload".to_string(),
                ));
            }
            let data_len = u32::from_le_bytes(payload[29..33].try_into().unwrap()) as usize;
            if offset + data_len > payload.len() {
                return Err(AudioTransferError::InvalidInput(
                    "Invalid chunk payload".to_string(),
                ));
            }
            return Ok(payload[offset..offset + data_len].to_vec());
        }

        let manifest = kat::decode_manifest_payload(&payload)?;

        let scan_addrs: Vec<String> = tx
            .outputs
            .as_ref()
            .map(|outs| {
                let mut uniq: Vec<String> = Vec::new();
                for o in outs.iter() {
                    let Some(a) = o.script_public_key_address.as_ref() else {
                        continue;
                    };
                    if a.trim().is_empty() {
                        continue;
                    }
                    if !uniq.iter().any(|x| x == a) {
                        uniq.push(a.clone());
                    }
                }
                uniq
            })
            .unwrap_or_default();

        if scan_addrs.is_empty() {
            return Err(AudioTransferError::KaspaRpc(
                "REST tx outputs missing scan address".to_string(),
            ));
        }

        let mut chunks: Vec<Option<Vec<u8>>> = vec![None; manifest.total_chunks as usize];
        let mut found_chunks: usize = 0;

        let limit: u32 = 500;

        for (addr_i, addr) in scan_addrs.iter().enumerate() {
            let mut page: u32 = 0;
            let mut before: Option<u64> = None;
            let mut stalled_pages: u32 = 0;

            while page < 5000 {
                Self::progress_line(&format!(
                    "REST scanning {} ({}/{}) | {}/{} chunks | page {}",
                    addr,
                    addr_i + 1,
                    scan_addrs.len(),
                    found_chunks,
                    chunks.len(),
                    page
                ));

                let prior_found = found_chunks;
                let (txs, next_before) =
                    rest::rest_get_address_txs_page(addr, limit, before).await?;
                if txs.is_empty() {
                    break;
                }
                for t in txs.iter() {
                    let Some(payload_hex) = t.payload.as_ref() else {
                        continue;
                    };
                    let Ok(p) = hex::decode(payload_hex) else {
                        continue;
                    };
                    if let Some((file_id, idx, total, offset)) = kat::try_decode_chunk_header(&p) {
                        if file_id == manifest.file_id && total == manifest.total_chunks {
                            if p.len() < 33 {
                                continue;
                            }
                            let data_len =
                                u32::from_le_bytes(p[29..33].try_into().unwrap()) as usize;
                            if offset + data_len > p.len() {
                                continue;
                            }
                            if (idx as usize) < chunks.len() {
                                if chunks[idx as usize].is_none() {
                                    found_chunks = found_chunks.saturating_add(1);
                                }
                                chunks[idx as usize] = Some(p[offset..offset + data_len].to_vec());
                            }
                        }
                    }
                }

                if chunks.iter().all(|c| c.is_some()) {
                    break;
                }

                if found_chunks == prior_found {
                    stalled_pages += 1;
                    if stalled_pages >= 5 {
                        break;
                    }
                } else {
                    stalled_pages = 0;
                }

                before = next_before;
                if before.is_none() {
                    break;
                }
                page += 1;
            }

            if chunks.iter().all(|c| c.is_some()) {
                break;
            }
        }

        Self::progress_end();

        if !chunks.iter().all(|c| c.is_some()) {
            return Err(AudioTransferError::KaspaRpc(
                "Unable to locate all chunks via api.kaspa.org".to_string(),
            ));
        }

        let mut out = Vec::with_capacity(manifest.total_size as usize);
        for (i, chunk) in chunks
            .iter()
            .enumerate()
            .take(manifest.total_chunks as usize)
        {
            let Some(c) = chunk.as_ref() else {
                return Err(AudioTransferError::KaspaRpc(format!("Missing chunk {i}")));
            };
            out.extend_from_slice(c);
        }
        out.truncate(manifest.total_size as usize);
        Ok(out)
    }

    pub async fn receive_audio(
        &self,
        tx_id: &str,
        start_block_hash: Option<&str>,
    ) -> Result<Vec<u8>> {
        let tx_id_str = tx_id;
        println!("Fetching transaction: {}", tx_id);
        let tx_id = TransactionId::from_str(tx_id_str)
            .map_err(|e| AudioTransferError::KaspaRpc(format!("Invalid transaction ID: {e}")))?;

        let mut rpc = self.rpc.clone();
        let mut reconnects: u32 = 0;

        let mut resolved_scan_anchors: Vec<String> = Vec::new();
        let mut resolved_payload: Option<Vec<u8>> = None;
        if start_block_hash.is_none() {
            if let Ok(tx) = rest::rest_get_tx(tx_id_str, None, false, true).await {
                resolved_scan_anchors = rest::rest_string_or_vec_to_vec(tx.block_hashes);
                if self.is_public_resolver {
                    if let Ok(p) = rest::rest_payload_hex_to_bytes(tx.payload) {
                        resolved_payload = Some(p);
                    }
                }
            }
        }

        let mut remaining_auto_anchors: Vec<String> = if start_block_hash.is_none() {
            resolved_scan_anchors.clone()
        } else {
            Vec::new()
        };

        let mut effective_start_block_hash: Option<String> =
            start_block_hash.map(|s| s.to_string());
        if effective_start_block_hash.is_none() && !remaining_auto_anchors.is_empty() {
            effective_start_block_hash = Some(remaining_auto_anchors.remove(0));
        }

        let payload = if let Some(p) = resolved_payload {
            p
        } else {
            loop {
                match self
                    .find_transaction_payload(&rpc, tx_id, effective_start_block_hash.as_deref())
                    .await
                {
                    Ok(p) => break p,
                    Err(e) => {
                        if self.is_public_resolver
                            && Self::is_resolver_disconnect_error(&e)
                            && reconnects < Self::RESOLVER_RECONNECT_ATTEMPTS
                        {
                            reconnects += 1;
                            Self::progress_end();
                            eprintln!(
                                "Info: public resolver disconnected during tx scan; reconnecting ({}/{})",
                                reconnects,
                                Self::RESOLVER_RECONNECT_ATTEMPTS
                            );
                            sleep(Duration::from_millis(
                                250u64.saturating_mul(reconnects as u64),
                            ))
                            .await;
                            let network_id = self
                                .resolver_network_id
                                .unwrap_or_else(|| NetworkId::new(NetworkType::Mainnet));
                            rpc = Self::connect_wrpc_via_resolver(network_id).await?;
                            continue;
                        }

                        if self.is_public_resolver {
                            eprintln!(
                                "Info: public resolver RPC failed during tx scan ({}); falling back to api.kaspa.org",
                                e
                            );
                            return self
                                .receive_audio_via_rest(
                                    tx_id_str,
                                    effective_start_block_hash.as_deref(),
                                )
                                .await;
                        }
                        if Self::should_use_rest_fallback(&e) {
                            return self
                                .receive_audio_via_rest(
                                    tx_id_str,
                                    effective_start_block_hash.as_deref(),
                                )
                                .await;
                        }
                        return Err(e);
                    }
                }
            }
        };

        if !kat::is_kat_payload(&payload) {
            return Ok(payload);
        }

        if payload.len() >= 5 && payload[4] == kat::KAT_TYPE_CHUNK {
            let Some((_file_id, _idx, _total, offset)) = kat::try_decode_chunk_header(&payload)
            else {
                return Err(AudioTransferError::InvalidInput(
                    "Invalid chunk payload".to_string(),
                ));
            };
            let data_len = u32::from_le_bytes(payload[29..33].try_into().unwrap()) as usize;
            return Ok(payload[offset..offset + data_len].to_vec());
        }

        let manifest = kat::decode_manifest_payload(&payload)?;

        let mut chunks: Vec<Option<Vec<u8>>> = vec![None; manifest.total_chunks as usize];
        let mut found_chunks: usize = 0;

        if let Ok(entries) = rpc
            .get_mempool_entries(true, false)
            .await
            .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))
        {
            for entry in entries {
                let p = entry.transaction.payload;
                if let Some((file_id, idx, total, offset)) = kat::try_decode_chunk_header(&p) {
                    if file_id == manifest.file_id && total == manifest.total_chunks {
                        let data_len = u32::from_le_bytes(p[29..33].try_into().unwrap()) as usize;
                        if (idx as usize) < chunks.len() {
                            if chunks[idx as usize].is_none() {
                                found_chunks = found_chunks.saturating_add(1);
                            }
                            chunks[idx as usize] = Some(p[offset..offset + data_len].to_vec());
                        }
                    }
                }
            }
        }

        let mut chunk_reconnects: u32 = 0;
        loop {
            let dag = rpc
                .get_block_dag_info()
                .await
                .map_err(|e| AudioTransferError::KaspaRpc(e.to_string()))?;

            let mut start_hash: RpcHash = if let Some(h) = effective_start_block_hash.as_deref() {
                RpcHash::from_str(h).map_err(|e| {
                    AudioTransferError::KaspaRpc(format!("Invalid start_block_hash: {e}"))
                })?
            } else {
                dag.pruning_point_hash
            };
            let page_limit: u64 = if self.is_public_resolver {
                if effective_start_block_hash.is_some() {
                    PUBLIC_VIRTUAL_CHAIN_PAGE_LIMIT_ANCHORED
                } else {
                    25
                }
            } else {
                VIRTUAL_CHAIN_PAGE_LIMIT
            };

            let mut page: u32 = 0;
            let mut used_user_start_hash = effective_start_block_hash.is_some();
            'scan_chunks: while page < 2000u32 {
                if !used_user_start_hash && page >= Self::MAX_RPC_SCAN_PAGES_FROM_PRUNING {
                    Self::progress_end();
                    break;
                }
                Self::progress_line(&format!(
                    "Scanning chunks | {}/{} | page {} | from {}{}",
                    found_chunks,
                    chunks.len(),
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
                    match rpc
                        .get_virtual_chain_from_block_v2(
                            start_hash,
                            Some(RpcDataVerbosityLevel::Full),
                            Some(page_limit),
                        )
                        .await
                    {
                        Ok(resp) => break resp,
                        Err(e) => {
                            let msg = e.to_string();
                            if self.is_public_resolver
                                && Self::is_resolver_disconnect_error(
                                    &AudioTransferError::KaspaRpc(msg.clone()),
                                )
                                && chunk_reconnects < Self::RESOLVER_RECONNECT_ATTEMPTS
                            {
                                chunk_reconnects += 1;
                                Self::progress_end();
                                eprintln!(
                                    "Info: public resolver disconnected during chunk scan; reconnecting ({}/{})",
                                    chunk_reconnects,
                                    Self::RESOLVER_RECONNECT_ATTEMPTS
                                );
                                sleep(Duration::from_millis(
                                    250u64.saturating_mul(chunk_reconnects as u64),
                                ))
                                .await;
                                let network_id = self
                                    .resolver_network_id
                                    .unwrap_or_else(|| NetworkId::new(NetworkType::Mainnet));
                                rpc = Self::connect_wrpc_via_resolver(network_id).await?;
                                continue;
                            }
                            if Self::should_use_rest_fallback(&AudioTransferError::KaspaRpc(
                                msg.clone(),
                            )) {
                                Self::progress_end();
                                eprintln!(
                                    "Info: RPC scan failed ({}); falling back to api.kaspa.org for retrieval",
                                    msg
                                );
                                return self
                                    .receive_audio_via_rest(
                                        tx_id_str,
                                        effective_start_block_hash.as_deref(),
                                    )
                                    .await;
                            }
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
                                continue 'scan_chunks;
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

                for block in response.chain_block_accepted_transactions.iter() {
                    for tx in block.accepted_transactions.iter() {
                        let Some(p) = tx.payload.as_ref() else {
                            continue;
                        };
                        if let Some((file_id, idx, total, offset)) = kat::try_decode_chunk_header(p)
                        {
                            if file_id == manifest.file_id && total == manifest.total_chunks {
                                let data_len =
                                    u32::from_le_bytes(p[29..33].try_into().unwrap()) as usize;
                                if (idx as usize) < chunks.len() {
                                    if chunks[idx as usize].is_none() {
                                        found_chunks = found_chunks.saturating_add(1);
                                    }
                                    chunks[idx as usize] =
                                        Some(p[offset..offset + data_len].to_vec());
                                }
                            }
                        }
                    }
                }

                if chunks.iter().all(|c| c.is_some()) {
                    Self::progress_end();
                    break;
                }

                let added = response.added_chain_block_hashes.as_ref();
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
                if start_block_hash.is_none() && !remaining_auto_anchors.is_empty() {
                    eprintln!(
                        "Info: chunks not found when scanning from start_block_hash; trying next scan anchor ({})",
                        remaining_auto_anchors[0]
                    );
                    effective_start_block_hash = Some(remaining_auto_anchors.remove(0));
                    continue;
                }
                eprintln!(
                    "Info: chunks not found when scanning from start_block_hash; retrying from pruning point {}",
                    dag.pruning_point_hash
                );
                effective_start_block_hash = None;
                continue;
            }

            Self::progress_end();
            break;
        }

        if chunks.iter().any(|c| c.is_none()) {
            eprintln!(
                "Info: unable to locate all chunks via node RPC scan; falling back to api.kaspa.org. Hint: for reliable RPC retrieval, pass receive --start-block-hash using an explorer 'Block hashes' value (scan anchor) rather than the txid."
            );
            return self
                .receive_audio_via_rest(tx_id_str, start_block_hash)
                .await;
        }

        let mut out = Vec::with_capacity(manifest.total_size as usize);
        for (i, chunk) in chunks
            .iter()
            .enumerate()
            .take(manifest.total_chunks as usize)
        {
            let Some(c) = chunk.as_ref() else {
                return Err(AudioTransferError::KaspaRpc(format!("Missing chunk {i}")));
            };
            out.extend_from_slice(c);
        }
        out.truncate(manifest.total_size as usize);
        Ok(out)
    }
}
