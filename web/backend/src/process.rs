use serde::Serialize;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

fn extract_64hex_hash(output: &str) -> Option<String> {
    for line in output.lines() {
        let t = line.trim();
        if t.len() == 64 && t.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Some(t.to_string());
        }
    }
    None
}

#[derive(Debug, Serialize)]
pub struct EstimateResult {
    pub chunk_count: u32,
    pub chunk_size: u32,
    pub manifest_fee_kas: f64,
    pub chunk_fees_kas: f64,
    pub total_network_fee_kas: f64,
    pub effective_cost_per_mib_kas: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SendResult {
    pub tx_id: String,
}

pub async fn run_estimate(
    file_path: &Path,
    service_private_key: &str,
    rpc_url: Option<&str>,
    storage_amount_kas: f64,
    kaspa_binary: &str,
) -> Result<EstimateResult, String> {
    let amount_arg = format!("{:.8}", storage_amount_kas);

    let mut cmd = Command::new(kaspa_binary);
    cmd.arg("estimate")
        .arg(file_path)
        .arg("--from-private-key")
        .arg(service_private_key);

    if let Some(u) = rpc_url {
        cmd.arg("--rpc-url").arg(u);
    }

    cmd.arg("--amount")
        .arg(&amount_arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "kaspa_audio_transfer estimate failed with status {}: {}",
            output.status, stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut chunk_count: u32 = 0;
    let mut chunk_size: u32 = 0;
    let mut manifest_fee_kas: f64 = 0.0;
    let mut chunk_fees_kas: f64 = 0.0;
    let mut total_network_fee_kas: f64 = 0.0;
    let mut effective_cost_per_mib_kas: Option<f64> = None;

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Chunking file into ") {
            // Format: "Chunking file into N chunk(s) of up to M bytes each"
            if let Some(rest) = trimmed.strip_prefix("Chunking file into ") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 5 {
                    if let Ok(n) = parts[0].parse::<u32>() {
                        chunk_count = n;
                    }
                    if let Ok(m) = parts[4].parse::<u32>() {
                        chunk_size = m;
                    }
                }
            }
        } else if trimmed.starts_with("Manifest:") {
            manifest_fee_kas = parse_kas_amount(trimmed).unwrap_or(0.0);
        } else if trimmed.starts_with("Chunks:") {
            chunk_fees_kas = parse_kas_amount(trimmed).unwrap_or(0.0);
        } else if trimmed.starts_with("Total:") {
            total_network_fee_kas = parse_kas_amount(trimmed).unwrap_or(0.0);
        } else if trimmed.starts_with("Effective cost:") {
            // Format: "Effective cost: X KAS per MiB"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                // Example split: ["Effective", "cost:", "0.12345678", "KAS", "per", "MiB"]
                if let Ok(v) = parts[2].parse::<f64>() {
                    effective_cost_per_mib_kas = Some(v);
                }
            }
        }
    }

    Ok(EstimateResult {
        chunk_count,
        chunk_size,
        manifest_fee_kas,
        chunk_fees_kas,
        total_network_fee_kas,
        effective_cost_per_mib_kas,
    })
}

pub async fn run_send(
    file_path: &Path,
    from_private_key: &str,
    rpc_url: Option<&str>,
    resume_from: Option<&str>,
    resume_output_index: u32,
    to_address: &str,
    amount_kas: f64,
    kaspa_binary: &str,
) -> Result<SendResult, String> {
    let amount_arg = format!("{:.8}", amount_kas);

    let mut cmd = Command::new(kaspa_binary);
    cmd.arg("send")
        .arg(file_path)
        .arg("--from-private-key")
        .arg(from_private_key);

    if let Some(u) = rpc_url {
        cmd.arg("--rpc-url").arg(u);
    }
    if let Some(r) = resume_from {
        cmd.arg("--resume-from").arg(r);
        cmd.arg("--resume-output-index").arg(resume_output_index.to_string());
    }

    cmd.arg("--to-address")
        .arg(to_address)
        .arg("--amount")
        .arg(&amount_arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "kaspa_audio_transfer send failed with status {}: {}",
            output.status, stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Transaction ID:") {
            let tx_id = rest.trim();
            if !tx_id.is_empty() {
                return Ok(SendResult {
                    tx_id: tx_id.to_string(),
                });
            }
        }
    }

    Err("kaspa_audio_transfer send succeeded but no Transaction ID found in output".to_string())
}

pub async fn run_receive(
    tx_id: &str,
    output_path: &Path,
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
    kaspa_binary: &str,
) -> Result<(), String> {
    let mut cmd = Command::new(kaspa_binary);
    cmd.arg("receive")
        .arg(tx_id)
        .arg("--output")
        .arg(output_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(u) = rpc_url {
        cmd.arg("--rpc-url").arg(u);
    }
    if let Some(h) = start_block_hash {
        cmd.arg("--start-block-hash").arg(h);
    }

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "kaspa_audio_transfer receive failed with status {}: {}",
            output.status, stderr
        ));
    }

    Ok(())
}

pub async fn run_tx_accepting_block_hash(
    tx_id: &str,
    rpc_url: Option<&str>,
    start_block_hash: Option<&str>,
    min_confirmations: u64,
    wait_secs: u64,
    kaspa_binary: &str,
) -> Result<Option<String>, String> {
    let mut cmd = Command::new(kaspa_binary);
    cmd.arg("tx-accepting-block-hash").arg(tx_id);

    if let Some(u) = rpc_url {
        cmd.arg("--rpc-url").arg(u);
    }
    if let Some(h) = start_block_hash {
        cmd.arg("--start-block-hash").arg(h);
    }
    cmd.arg("--min-confirmations")
        .arg(min_confirmations.to_string());
    cmd.arg("--wait-secs").arg(wait_secs.to_string());

    let output = cmd
        .output()
        .await
        .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "kaspa_audio_transfer tx-accepting-block-hash failed (bin='{}') with status {}: {}",
            kaspa_binary,
            output.status,
            stderr
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(extract_64hex_hash(&stdout))
}

fn parse_kas_amount(line: &str) -> Option<f64> {
    // Expected formats like "Manifest: X KAS", "Chunks:   Y KAS", "Total:    Z KAS"
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in parts {
        if let Ok(v) = part.parse::<f64>() {
            return Some(v);
        }
    }
    None
}
