//! REST API fallback for transaction retrieval (api.kaspa.org).

use reqwest::Url;
use serde::Deserialize;

use crate::error::{AudioTransferError, Result};

#[derive(Debug, Deserialize)]
pub struct RestTxOutput {
    pub script_public_key_address: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RestStringOrVec {
    One(String),
    Many(Vec<String>),
}

pub fn rest_string_or_vec_to_vec(v: Option<RestStringOrVec>) -> Vec<String> {
    match v {
        Some(RestStringOrVec::One(s)) => vec![s],
        Some(RestStringOrVec::Many(v)) => v,
        None => vec![],
    }
}

#[derive(Debug, Deserialize)]
pub struct RestTxModel {
    pub payload: Option<String>,
    pub outputs: Option<Vec<RestTxOutput>>,
    #[serde(
        default,
        alias = "blockHash",
        alias = "block_hash",
        alias = "blockHashes",
        alias = "block_hashes"
    )]
    pub block_hashes: Option<RestStringOrVec>,
}

#[derive(Debug, Deserialize)]
pub struct RestAddressTxModel {
    pub payload: Option<String>,
}

pub fn rest_payload_hex_to_bytes(payload_hex: Option<String>) -> Result<Vec<u8>> {
    let Some(p) = payload_hex else {
        return Err(AudioTransferError::KaspaRpc(
            "REST transaction payload missing".to_string(),
        ));
    };
    hex::decode(&p)
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST payload hex decode failed: {e}")))
}

pub async fn rest_get_tx(
    tx_id: &str,
    block_hash: Option<&str>,
    inputs: bool,
    outputs: bool,
) -> Result<RestTxModel> {
    let mut url = Url::parse("https://api.kaspa.org")
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST url parse failed: {e}")))?;
    {
        let mut seg = url
            .path_segments_mut()
            .map_err(|_| AudioTransferError::KaspaRpc("REST url cannot be a base".to_string()))?;
        seg.push("transactions");
        seg.push(tx_id);
    }
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("inputs", if inputs { "true" } else { "false" });
        q.append_pair("outputs", if outputs { "true" } else { "false" });
        if let Some(h) = block_hash {
            q.append_pair("blockHash", h);
        }
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST client build failed: {e}")))?;

    let resp = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST request failed: {e}")))?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp
            .text()
            .await
            .unwrap_or_else(|_| "<no body>".to_string());
        return Err(AudioTransferError::KaspaRpc(format!(
            "REST GET {} failed: {}: {}",
            url, status, text
        )));
    }
    resp.json::<RestTxModel>()
        .await
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST json parse failed: {e}")))
}

pub async fn rest_get_address_txs_page(
    address: &str,
    limit: u32,
    before: Option<u64>,
) -> Result<(Vec<RestAddressTxModel>, Option<u64>)> {
    let mut url = Url::parse("https://api.kaspa.org")
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST url parse failed: {e}")))?;
    {
        let mut seg = url
            .path_segments_mut()
            .map_err(|_| AudioTransferError::KaspaRpc("REST url cannot be a base".to_string()))?;
        seg.push("addresses");
        seg.push(address);
        seg.push("full-transactions-page");
    }
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("limit", &limit.to_string());
        q.append_pair("fields", "transaction_id,payload");
        if let Some(b) = before.filter(|v| *v > 0) {
            q.append_pair("before", &b.to_string());
        }
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .build()
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST client build failed: {e}")))?;

    let resp = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST request failed: {e}")))?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp
            .text()
            .await
            .unwrap_or_else(|_| "<no body>".to_string());
        return Err(AudioTransferError::KaspaRpc(format!(
            "REST GET {} failed: {}: {}",
            url, status, text
        )));
    }

    let next_before = resp
        .headers()
        .get("x-next-page-before")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .filter(|v| *v > 0);

    let txs = resp
        .json::<Vec<RestAddressTxModel>>()
        .await
        .map_err(|e| AudioTransferError::KaspaRpc(format!("REST json parse failed: {e}")))?;

    Ok((txs, next_before))
}
