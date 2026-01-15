pub mod audio;
pub mod cli;
pub mod error;
pub mod kaspa;

use crate::error::Result;
use crate::kaspa::KaspaClient;

pub async fn send_bytes(
    data: &[u8],
    from_private_key: &str,
    rpc_url: Option<&str>,
    resume_from: Option<&str>,
    resume_output_index: u32,
    feerate: Option<f64>,
    fee_multiplier: Option<f64>,
    to_address: &str,
    amount: f64,
) -> Result<String> {
    let kaspa = KaspaClient::new(rpc_url).await?;
    let tx_id = kaspa
        .send_audio_signed(
            data,
            from_private_key,
            to_address,
            amount,
            resume_from,
            resume_output_index,
            feerate,
            fee_multiplier,
        )
        .await?;
    Ok(tx_id)
}

pub async fn receive_bytes(
    tx_id: &str,
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
) -> Result<Vec<u8>> {
    let kaspa = KaspaClient::new(rpc_url).await?;
    kaspa.receive_audio(tx_id, start_block_hash).await
}

pub async fn get_network_info(rpc_url: Option<&str>) -> Result<String> {
    let kaspa = KaspaClient::new(rpc_url).await?;
    kaspa.get_network_info().await
}

pub async fn resolve_tx_accepting_block_hash(
    tx_id: &str,
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
    min_confirmations: u64,
    wait_secs: u64,
) -> Result<Option<String>> {
    let kaspa = KaspaClient::new(rpc_url).await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(wait_secs);

    loop {
        let found = kaspa
            .get_tx_accepting_block_hash(tx_id, start_block_hash, min_confirmations)
            .await?;
        if found.is_some() {
            return Ok(found);
        }

        if wait_secs == 0 {
            return Ok(None);
        }
        if std::time::Instant::now() >= deadline {
            return Ok(None);
        }
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    }
}
