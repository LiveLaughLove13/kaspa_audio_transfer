#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wallet_vault;
mod wallet_kaspa;

use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

use std::io::{BufRead, Read};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Window;
use tauri::api::path::data_dir;

const DEFAULT_WALLET_DERIVATION_PATH: &str = "m/44'/111111'/0'/0/0";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AppSettings {
    #[serde(default)]
    node_bundle_dir: Option<String>,
}

fn app_data_root_dir() -> Result<PathBuf, String> {
    let mut base = data_dir().ok_or_else(|| "unable to resolve OS data_dir".to_string())?;
    base.push("KaspaAudioTransfer");
    std::fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    Ok(base)
}

fn settings_file_path() -> Result<PathBuf, String> {
    Ok(app_data_root_dir()?.join("settings.json"))
}

fn load_settings() -> Result<AppSettings, String> {
    let p = settings_file_path()?;
    match std::fs::read_to_string(&p) {
        Ok(s) => serde_json::from_str(&s).map_err(|e| format!("failed to parse settings.json: {e}")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(AppSettings::default()),
        Err(e) => Err(format!("failed to read settings.json: {e}")),
    }
}

fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let p = settings_file_path()?;
    let s = serde_json::to_string_pretty(settings).map_err(|e| format!("failed to serialize settings.json: {e}"))?;
    std::fs::write(&p, s).map_err(|e| format!("failed to write settings.json: {e}"))
}

fn validate_node_bundle_dir(dir: &Path) -> Result<(), String> {
    if !dir.exists() {
        return Err("selected folder does not exist".to_string());
    }
    if !dir.is_dir() {
        return Err("selected path is not a folder".to_string());
    }

    let mut missing: Vec<String> = Vec::new();

    let kaspad_ok = dir.join("kaspad.exe").exists() || dir.join("kaspad").exists();
    if !kaspad_ok {
        missing.push("kaspad.exe".to_string());
    }

    let bridge_ok = dir.join("stratum-bridge.exe").exists() || dir.join("stratum-bridge").exists();
    if !bridge_ok {
        missing.push("stratum-bridge.exe".to_string());
    }

    if !dir.join("config.yaml").exists() {
        missing.push("config.yaml".to_string());
    }

    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "folder is missing required files:\n{}",
            missing.join("\n")
        ))
    }
}

#[tauri::command(rename_all = "camelCase")]
fn node_bundle_get_dir() -> Result<Option<String>, String> {
    let settings = load_settings()?;
    Ok(settings.node_bundle_dir)
}

#[tauri::command(rename_all = "camelCase")]
fn node_bundle_set_dir(dir: Option<String>) -> Result<(), String> {
    let mut settings = load_settings()?;

    match dir.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(d) => {
            let p = PathBuf::from(d);
            validate_node_bundle_dir(&p)?;
            settings.node_bundle_dir = Some(p.to_string_lossy().to_string());
        }
        None => {
            settings.node_bundle_dir = None;
        }
    }

    save_settings(&settings)
}

#[derive(Debug, Deserialize)]
struct KnsOwnerData {
    owner: String,
}

#[derive(Debug, Deserialize)]
struct KnsOwnerResponse {
    success: bool,
    data: Option<KnsOwnerData>,
    message: Option<String>,
    error: Option<String>,
}

fn kns_env_from_network(network: Option<&str>) -> Result<&'static str, String> {
    match network.unwrap_or("mainnet").trim().to_ascii_lowercase().as_str() {
        "mainnet" => Ok("mainnet"),
        "tn10" | "testnet" | "testnet10" => Ok("tn10"),
        other => Err(format!("unsupported KNS network: {other}")),
    }
}

#[tauri::command(rename_all = "camelCase")]
async fn kns_domain_owner(domain: String, network: Option<String>) -> Result<String, String> {
    let raw = domain.trim();
    if raw.is_empty() {
        return Err("domain is empty".to_string());
    }

    let env = kns_env_from_network(network.as_deref())?;

    let mut d = raw.to_string();
    if !d.to_ascii_lowercase().ends_with(".kas") {
        d.push_str(".kas");
    }

    let encoded = urlencoding::encode(&d);
    let url = format!("https://api.knsdomains.org/{env}/api/v1/{encoded}/owner");

    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("failed to build http client: {e}"))?;

    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("failed to call KNS API: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_else(|_| "<no body>".to_string());
        return Err(format!("KNS API error {}: {}", status.as_u16(), body));
    }

    let parsed = resp
        .json::<KnsOwnerResponse>()
        .await
        .map_err(|e| format!("failed to parse KNS response: {e}"))?;

    if parsed.success {
        let owner = parsed
            .data
            .map(|d| d.owner)
            .unwrap_or_default()
            .trim()
            .to_string();
        if owner.is_empty() {
            return Err("KNS response missing owner".to_string());
        }
        Ok(owner)
    } else {
        Err(parsed
            .error
            .or(parsed.message)
            .unwrap_or_else(|| "KNS resolution failed".to_string()))
    }
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profiles_list() -> Result<Vec<wallet_vault::WalletProfileInfo>, String> {
    wallet_vault::list_profiles()
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_create_mnemonic(username: String, password: String, word_count: u32) -> Result<String, String> {
    wallet_vault::create_profile_mnemonic(&username, &password, word_count)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_import_mnemonic(
    username: String,
    password: String,
    phrase: String,
    mnemonic_password: Option<String>,
) -> Result<(), String> {
    wallet_vault::import_profile_mnemonic(
        &username,
        &password,
        &phrase,
        mnemonic_password.as_deref(),
    )
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_import_private_key(username: String, password: String, private_key_hex: String) -> Result<(), String> {
    wallet_vault::import_profile_private_key(&username, &password, &private_key_hex)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_delete(username: String) -> Result<(), String> {
    wallet_vault::delete_profile(&username)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profiles_clear_all() -> Result<(), String> {
    wallet_vault::clear_all_profiles()
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_unlock(username: String, password: String) -> Result<(), String> {
    wallet_vault::unlock_profile(&username, &password)
}

#[tauri::command]
fn wallet_lock() -> Result<(), String> {
    wallet_vault::lock_wallet();
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_unlocked_username() -> Result<Option<String>, String> {
    Ok(wallet_vault::get_unlocked_username())
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_derive_receive_address(network: String, derivation_path: String) -> Result<String, String> {
    wallet_vault::derive_receive_address(&network, &derivation_path)
}

#[tauri::command]
fn wallet_debug_unlocked_material_fingerprint() -> Result<String, String> {
    wallet_vault::debug_unlocked_material_fingerprint()
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_get_balance(network: String, derivation_path: String, rpc_url: Option<String>) -> Result<f64, String> {
    wallet_kaspa::wallet_get_balance_kas(&network, &derivation_path, rpc_url.as_deref()).await
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_send_kas(
    network: String,
    derivation_path: String,
    rpc_url: Option<String>,
    to_address: String,
    amount_kas: f64,
) -> Result<String, String> {
    wallet_kaspa::wallet_send_kas(
        &network,
        &derivation_path,
        rpc_url.as_deref(),
        &to_address,
        amount_kas,
    )
    .await
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_send_file_path(
    window: Window,
    account_id: Option<String>,
    file_path: String,
    to_address: String,
    amount_kas: f64,
    rpc_url: Option<String>,
    resume_from: Option<String>,
    resume_output_index: Option<u32>,
    _file_name: Option<String>,
    from_private_key: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _ = account_id;

        let to_address = to_address.trim();
        if to_address.is_empty() {
            return Err("missing toAddress".to_string());
        }

        let file_path = file_path.trim();
        if file_path.is_empty() {
            return Err("missing filePath".to_string());
        }
        if std::fs::metadata(file_path).is_err() {
            return Err("filePath not found".to_string());
        }

        let from_private_key = match from_private_key.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(v) => v.to_string(),
            None => {
                let keypair = wallet_vault::derive_keypair_for_path("mainnet", DEFAULT_WALLET_DERIVATION_PATH)?;
                let sk = keypair.secret_key();
                bytes_to_lower_hex(&sk.secret_bytes())
            }
        };

        if !amount_kas.is_finite() || amount_kas < 0.0 {
            return Err("invalid amountKas".to_string());
        }

        let exe = find_kaspa_audio_transfer_binary()?;
        let mut cmd = Command::new(exe);
        cmd.arg("send")
            .arg(file_path)
            .arg("--from-private-key")
            .arg(from_private_key)
            .arg("--to-address")
            .arg(to_address)
            .arg("--amount")
            .arg(format!("{:.8}", amount_kas))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(windows)]
        set_no_window(&mut cmd);

        if let Some(u) = rpc_url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--rpc-url").arg(u);
        }
        if let Some(r) = resume_from.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--resume-from").arg(r);
            cmd.arg("--resume-output-index")
                .arg(resume_output_index.unwrap_or(1).to_string());
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "failed to capture stdout".to_string())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "failed to capture stderr".to_string())?;

        let stderr_buf = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let stderr_buf2 = stderr_buf.clone();
        let stderr_handle = std::thread::spawn(move || {
            let mut r = std::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(n) = r.read_line(&mut line) {
                if n == 0 {
                    break;
                }
                if let Ok(mut buf) = stderr_buf2.lock() {
                    buf.push_str(line.trim_end());
                    buf.push('\n');
                }
                line.clear();
            }
        });

        let mut total_chunks: Option<u32> = None;
        let mut submitted_chunks: u32 = 0;
        let mut txid: Option<String> = None;

        let mut r = std::io::BufReader::new(stdout);
        let mut line = String::new();
        while let Ok(n) = r.read_line(&mut line) {
            if n == 0 {
                break;
            }
            let trimmed = line.trim();

            if txid.is_none() {
                if let Some(rest) = trimmed.strip_prefix("Transaction ID:") {
                    let v = rest.trim();
                    if !v.is_empty() {
                        txid = Some(v.to_string());
                    }
                }
            }

            if total_chunks.is_none() {
                if let Some(t) = parse_total_chunks_from_line(trimmed) {
                    total_chunks = Some(t);
                    let _ = window.emit(
                        "kaspa_send_progress",
                        KaspaSendProgressEvent {
                            submitted_chunks,
                            total_chunks,
                        },
                    );
                }
            }

            if let Some((done, t)) = parse_submitted_chunks_from_line(trimmed) {
                submitted_chunks = done;
                if total_chunks.is_none() {
                    total_chunks = t;
                }
                let _ = window.emit(
                    "kaspa_send_progress",
                    KaspaSendProgressEvent {
                        submitted_chunks,
                        total_chunks,
                    },
                );
            }

            line.clear();
        }

        let status = child
            .wait()
            .map_err(|e| format!("failed waiting for kaspa_audio_transfer: {e}"))?;

        let _ = stderr_handle.join();

        if !status.success() {
            let stderr = stderr_buf.lock().unwrap_or_else(|e| e.into_inner());
            let stderr = stderr.trim();
            return Err(format!(
                "kaspa_audio_transfer send failed with status {}: {}",
                status,
                if stderr.is_empty() { "<no stderr>" } else { stderr }
            ));
        }

        if let Some(total) = total_chunks {
            let _ = window.emit(
                "kaspa_send_progress",
                KaspaSendProgressEvent {
                    submitted_chunks: total,
                    total_chunks: Some(total),
                },
            );
        }

        txid.ok_or_else(|| "send succeeded but no Transaction ID found in output".to_string())
    })
    .await
    .map_err(|e| format!("send task join error: {e}"))?
}

#[cfg(windows)]
fn set_no_window(cmd: &mut Command) {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    cmd.creation_flags(CREATE_NO_WINDOW);
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_receive_file(
    window: Window,
    tx_id: String,
    output_path: String,
    rpc_url: Option<String>,
    start_block_hash: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let tx_id = tx_id.trim();
        if tx_id.is_empty() {
            return Err("missing txId".to_string());
        }

        let output_path = output_path.trim();
        if output_path.is_empty() {
            return Err("missing outputPath".to_string());
        }

        let exe = find_kaspa_audio_transfer_binary()?;
        let mut cmd = Command::new(exe);
        cmd.arg("receive")
            .arg(tx_id)
            .arg("--output")
            .arg(output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(windows)]
        set_no_window(&mut cmd);

        if let Some(u) = rpc_url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--rpc-url").arg(u);
        }
        if let Some(h) = start_block_hash.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--start-block-hash").arg(h);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "failed to capture stdout".to_string())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "failed to capture stderr".to_string())?;

        let stdout_buf = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let stderr_buf = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let last_progress: std::sync::Arc<std::sync::Mutex<Option<(u32, u32)>>> =
            std::sync::Arc::new(std::sync::Mutex::new(None));

        let w1 = window.clone();
        let out_buf1 = stdout_buf.clone();
        let lp1 = last_progress.clone();
        let stdout_handle = std::thread::spawn(move || {
            pump_receive_output(stdout, w1, out_buf1, lp1);
        });

        let w2 = window.clone();
        let err_buf2 = stderr_buf.clone();
        let lp2 = last_progress.clone();
        let stderr_handle = std::thread::spawn(move || {
            pump_receive_output(stderr, w2, err_buf2, lp2);
        });

        let status = child
            .wait()
            .map_err(|e| format!("failed waiting for kaspa_audio_transfer: {e}"))?;

        let _ = stdout_handle.join();
        let _ = stderr_handle.join();

        if !status.success() {
            let stderr = stderr_buf.lock().unwrap_or_else(|e| e.into_inner());
            let stdout = stdout_buf.lock().unwrap_or_else(|e| e.into_inner());
            let stderr = stderr.trim();
            let stdout = stdout.trim();
            return Err(format!(
                "kaspa_audio_transfer receive failed with status {}: {}{}{}",
                status,
                if stderr.is_empty() { "" } else { stderr },
                if !stderr.is_empty() && !stdout.is_empty() { "\n" } else { "" },
                if stdout.is_empty() { "" } else { stdout }
            ));
        }

        let (_found, total) = last_progress
            .lock()
            .ok()
            .and_then(|g| *g)
            .unwrap_or((1, 1));
        let total = total.max(1);
        let _ = window.emit(
            "kaspa_receive_progress",
            KaspaReceiveProgressEvent {
                found_chunks: total,
                total_chunks: Some(total),
            },
        );

        Ok(output_path.to_string())
    })
    .await
    .map_err(|e| format!("receive task join error: {e}"))?
}

fn parse_receive_progress_line(line: &str) -> Option<(u32, u32)> {
    let trimmed = line.trim();

    if let Some(rest) = trimmed.strip_prefix("Scanning chunks | ") {
        let first = rest.split('|').next().unwrap_or("").trim();
        if let Some((a, b)) = first.split_once('/') {
            let found = a.trim().parse::<u32>().ok()?;
            let total = b.trim().parse::<u32>().ok()?;
            return Some((found, total));
        }
    }

    if let Some(pos) = trimmed.find("| ") {
        let after = &trimmed[pos + 2..];
        let first = after.split('|').next().unwrap_or("").trim();
        if let Some((a, b)) = first.split_once('/') {
            if let Some(rest_b) = b.split_whitespace().next() {
                let found = a.trim().parse::<u32>().ok()?;
                let total = rest_b.trim().parse::<u32>().ok()?;
                if trimmed.to_lowercase().contains("rest scanning") {
                    return Some((found, total));
                }
            }
        }
    }

    None
}

fn pump_receive_output<R: Read>(
    mut r: R,
    window: Window,
    buf: std::sync::Arc<std::sync::Mutex<String>>,
    last_progress: std::sync::Arc<std::sync::Mutex<Option<(u32, u32)>>>,
) {
    let mut tmp = [0u8; 4096];
    let mut pending: Vec<u8> = Vec::new();

    loop {
        let n = match r.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        if let Ok(mut b) = buf.lock() {
            b.push_str(&String::from_utf8_lossy(&tmp[..n]));
        }

        pending.extend_from_slice(&tmp[..n]);

        loop {
            let pos = pending
                .iter()
                .position(|b| *b == b'\n' || *b == b'\r');
            let Some(i) = pos else { break; };

            let line_bytes: Vec<u8> = pending.drain(..=i).collect();
            let line = String::from_utf8_lossy(&line_bytes);

            if let Some((found, total)) = parse_receive_progress_line(&line) {
                if let Ok(mut lp) = last_progress.lock() {
                    *lp = Some((found, total));
                }
                let _ = window.emit(
                    "kaspa_receive_progress",
                    KaspaReceiveProgressEvent {
                        found_chunks: found,
                        total_chunks: Some(total),
                    },
                );
            }
        }
    }
}

#[derive(Clone, Serialize)]
struct KaspaSendProgressEvent {
    submitted_chunks: u32,
    total_chunks: Option<u32>,
}

#[derive(Clone, Serialize)]
struct KaspaReceiveProgressEvent {
    found_chunks: u32,
    total_chunks: Option<u32>,
}

#[derive(Clone, Serialize)]
struct WalletStatus {
    unlocked_account_id: Option<String>,
}

#[tauri::command]
fn wallet_status() -> Result<WalletStatus, String> {
    Ok(WalletStatus {
        unlocked_account_id: None,
    })
}

fn sanitize_filename(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "upload.bin".to_string();
    }

    let base = trimmed
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(trimmed)
        .trim();

    let mut out = String::with_capacity(base.len());
    for ch in base.chars() {
        let ok = matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' | ' ');
        if ok {
            out.push(ch);
        } else {
            out.push('_');
        }
    }

    let cleaned = out.trim_matches([' ', '.']).to_string();
    if cleaned.is_empty() {
        "upload.bin".to_string()
    } else if cleaned.len() > 180 {
        cleaned.chars().take(180).collect()
    } else {
        cleaned
    }
}

fn find_kaspa_audio_transfer_binary() -> Result<PathBuf, String> {
    if let Ok(v) = std::env::var("KASPA_AUDIO_TRANSFER_BIN") {
        let p = PathBuf::from(v);
        if std::fs::metadata(&p).is_ok() {
            return Ok(p);
        }
        return Err("KASPA_AUDIO_TRANSFER_BIN is set but points to a missing file".to_string());
    }

    let exe_name = if cfg!(windows) {
        "kaspa_audio_transfer.exe"
    } else {
        "kaspa_audio_transfer"
    };

    // This file lives at: desktop/src-tauri/src/main.rs
    // Repo root is 2 levels up from CARGO_MANIFEST_DIR (desktop/src-tauri)
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent() // desktop
        .and_then(|p| p.parent()) // repo root
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.clone());

    let debug = repo_root.join("target").join("debug").join(exe_name);
    if std::fs::metadata(&debug).is_ok() {
        return Ok(debug);
    }
    let release = repo_root.join("target").join("release").join(exe_name);
    if std::fs::metadata(&release).is_ok() {
        return Ok(release);
    }

    Err("kaspa_audio_transfer binary not found. Build it or set KASPA_AUDIO_TRANSFER_BIN".to_string())
}

fn decode_b64_payload(payload: &str) -> Result<Vec<u8>, String> {
    let trimmed = payload.trim();
    if trimmed.is_empty() {
        return Err("file_b64 is empty".to_string());
    }

    let b64 = if let Some((_, rest)) = trimmed.split_once(",") {
        // Handle data URLs: data:...;base64,XXXX
        if trimmed.to_ascii_lowercase().contains("base64") {
            rest
        } else {
            trimmed
        }
    } else {
        trimmed
    };

    general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| format!("invalid base64 payload: {e}"))
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut out, "{:02x}", b);
    }
    out
}

fn parse_total_chunks_from_line(line: &str) -> Option<u32> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("Chunking file into ") {
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if !parts.is_empty() {
            return parts[0].parse::<u32>().ok();
        }
    }
    None
}

fn parse_submitted_chunks_from_line(line: &str) -> Option<(u32, Option<u32>)> {
    // Example: "Submitted chunk 3/42:" or "Submitted chunk 3/42"
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("Submitted chunk ")?;
    let first = rest.split_whitespace().next().unwrap_or("");
    let frac = first.trim_end_matches(':');
    let (a, b) = frac.split_once('/')?;
    let done = a.parse::<u32>().ok()?;
    let total = b.parse::<u32>().ok();
    Some((done, total))
}

#[tauri::command(rename_all = "camelCase")]
fn studio_temp_path(file_name: String) -> Result<String, String> {
    let safe = sanitize_filename(&file_name);
    let mut dir = std::env::temp_dir();
    dir.push("KaspaAudioTransfer");
    dir.push("studio");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let p = dir.join(safe);
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn ffmpeg_transcode(input_path: String, output_path: String, kind: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let input_path = input_path.trim();
        let output_path = output_path.trim();
        if input_path.is_empty() {
            return Err("inputPath is empty".to_string());
        }
        if output_path.is_empty() {
            return Err("outputPath is empty".to_string());
        }
        if std::fs::metadata(input_path).is_err() {
            return Err("inputPath not found".to_string());
        }

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y").arg("-i").arg(input_path);

        match kind.trim().to_ascii_lowercase().as_str() {
            "mp3" => {
                cmd.arg("-vn")
                    .arg("-c:a")
                    .arg("libmp3lame")
                    .arg("-q:a")
                    .arg("2")
                    .arg(output_path);
            }
            "wav" => {
                cmd.arg("-vn")
                    .arg("-c:a")
                    .arg("pcm_s16le")
                    .arg("-ar")
                    .arg("44100")
                    .arg("-ac")
                    .arg("2")
                    .arg(output_path);
            }
            "mp4" => {
                cmd.arg("-c:v")
                    .arg("libx264")
                    .arg("-preset")
                    .arg("veryfast")
                    .arg("-crf")
                    .arg("23")
                    .arg("-c:a")
                    .arg("aac")
                    .arg("-b:a")
                    .arg("192k")
                    .arg("-movflags")
                    .arg("+faststart")
                    .arg(output_path);
            }
            _ => return Err("kind must be one of: mp3, wav, mp4".to_string()),
        }

        #[cfg(windows)]
        set_no_window(&mut cmd);

        let out = cmd
            .output()
            .map_err(|e| format!("failed to run ffmpeg (is it installed and on PATH?): {e}"))?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stderr = stderr.trim();
            return Err(format!(
                "ffmpeg failed with status {}: {}",
                out.status,
                if stderr.is_empty() { "<no stderr>" } else { stderr }
            ));
        }

        Ok(())
    })
    .await
    .map_err(|e| format!("ffmpeg task join error: {e}"))?
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_core_send_file_b64(
    window: Window,
    account_id: Option<String>,
    file_b64: String,
    to_address: String,
    amount_kas: f64,
    rpc_url: Option<String>,
    resume_from: Option<String>,
    resume_output_index: Option<u32>,
    file_name: Option<String>,
    from_private_key: Option<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let _ = account_id;

        let to_address = to_address.trim();
        if to_address.is_empty() {
            return Err("missing toAddress".to_string());
        }

        let from_private_key = match from_private_key.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(v) => v.to_string(),
            None => {
                // Use the unlocked local wallet vault if available.
                // Network does not affect key derivation; we pass a valid value for validation.
                let keypair = wallet_vault::derive_keypair_for_path("mainnet", DEFAULT_WALLET_DERIVATION_PATH)?;
                let sk = keypair.secret_key();
                bytes_to_lower_hex(&sk.secret_bytes())
            }
        };

        if !amount_kas.is_finite() || amount_kas < 0.0 {
            return Err("invalid amountKas".to_string());
        }

        let bytes = decode_b64_payload(&file_b64)?;
        let safe_name = sanitize_filename(file_name.as_deref().unwrap_or("upload.bin"));

        let mut out_path = std::env::temp_dir();
        let unique = format!("kaspa_audio_transfer_{}_{}", std::process::id(), safe_name);
        out_path.push(unique);
        std::fs::write(&out_path, &bytes).map_err(|e| format!("failed to write temp file: {e}"))?;

        let exe = find_kaspa_audio_transfer_binary()?;
        let mut cmd = Command::new(exe);
        cmd.arg("send")
            .arg(&out_path)
            .arg("--from-private-key")
            .arg(from_private_key)
            .arg("--to-address")
            .arg(to_address)
            .arg("--amount")
            .arg(format!("{:.8}", amount_kas))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(windows)]
        set_no_window(&mut cmd);

        if let Some(u) = rpc_url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--rpc-url").arg(u);
        }
        if let Some(r) = resume_from.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            cmd.arg("--resume-from").arg(r);
            cmd.arg("--resume-output-index")
                .arg(resume_output_index.unwrap_or(1).to_string());
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("failed to run kaspa_audio_transfer: {e}"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "failed to capture stdout".to_string())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "failed to capture stderr".to_string())?;

        let stderr_buf = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let stderr_buf2 = stderr_buf.clone();
        let stderr_handle = std::thread::spawn(move || {
            let mut r = std::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(n) = r.read_line(&mut line) {
                if n == 0 {
                    break;
                }
                if let Ok(mut buf) = stderr_buf2.lock() {
                    buf.push_str(line.trim_end());
                    buf.push('\n');
                }
                line.clear();
            }
        });

        let mut total_chunks: Option<u32> = None;
        let mut submitted_chunks: u32 = 0;
        let mut txid: Option<String> = None;

        let mut r = std::io::BufReader::new(stdout);
        let mut line = String::new();
        while let Ok(n) = r.read_line(&mut line) {
            if n == 0 {
                break;
            }
            let trimmed = line.trim();

            if txid.is_none() {
                if let Some(rest) = trimmed.strip_prefix("Transaction ID:") {
                    let v = rest.trim();
                    if !v.is_empty() {
                        txid = Some(v.to_string());
                    }
                }
            }

            if total_chunks.is_none() {
                if let Some(t) = parse_total_chunks_from_line(trimmed) {
                    total_chunks = Some(t);
                    let _ = window.emit(
                        "kaspa_send_progress",
                        KaspaSendProgressEvent {
                            submitted_chunks,
                            total_chunks,
                        },
                    );
                }
            }

            if let Some((done, t)) = parse_submitted_chunks_from_line(trimmed) {
                submitted_chunks = done;
                if total_chunks.is_none() {
                    total_chunks = t;
                }
                let _ = window.emit(
                    "kaspa_send_progress",
                    KaspaSendProgressEvent {
                        submitted_chunks,
                        total_chunks,
                    },
                );
            }

            line.clear();
        }

        let status = child
            .wait()
            .map_err(|e| format!("failed waiting for kaspa_audio_transfer: {e}"))?;

        let _ = std::fs::remove_file(&out_path);
        let _ = stderr_handle.join();

        if !status.success() {
            let stderr = stderr_buf.lock().unwrap_or_else(|e| e.into_inner());
            let stderr = stderr.trim();
            return Err(format!(
                "kaspa_audio_transfer send failed with status {}: {}",
                status,
                if stderr.is_empty() { "<no stderr>" } else { stderr }
            ));
        }

        if let Some(total) = total_chunks {
            let _ = window.emit(
                "kaspa_send_progress",
                KaspaSendProgressEvent {
                    submitted_chunks: total,
                    total_chunks: Some(total),
                },
            );
        }

        txid.ok_or_else(|| "send succeeded but no Transaction ID found in output".to_string())
    })
    .await
    .map_err(|e| format!("send task join error: {e}"))?
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_send_file_b64(
    window: Window,
    account_id: Option<String>,
    file_b64: String,
    to_address: String,
    amount_kas: f64,
    rpc_url: Option<String>,
    resume_from: Option<String>,
    resume_output_index: Option<u32>,
    file_name: Option<String>,
    from_private_key: Option<String>,
) -> Result<String, String> {
    wallet_core_send_file_b64(
        window,
        account_id,
        file_b64,
        to_address,
        amount_kas,
        rpc_url,
        resume_from,
        resume_output_index,
        file_name,
        from_private_key,
    )
    .await
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("path is empty".to_string());
    }
    if std::fs::metadata(&path).is_err() {
        return Err("path not found".to_string());
    }

    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("explorer.exe");
        c.arg(&path);
        c
    } else if cfg!(target_os = "macos") {
        let mut c = Command::new("open");
        c.arg(&path);
        c
    } else {
        let mut c = Command::new("xdg-open");
        c.arg(&path);
        c
    };

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to open file: {e}"))?;

    Ok(())
}

#[tauri::command]
fn reveal_in_folder(path: String) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("path is empty".to_string());
    }
    if std::fs::metadata(&path).is_err() {
        return Err("path not found".to_string());
    }

    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("explorer.exe");
        c.arg("/select,").arg(&path);
        c
    } else if cfg!(target_os = "macos") {
        let mut c = Command::new("open");
        c.arg("-R").arg(&path);
        c
    } else {
        let p = std::path::Path::new(&path);
        let dir = p.parent().unwrap_or(p);
        let mut c = Command::new("xdg-open");
        c.arg(dir);
        c
    };

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("failed to reveal file: {e}"))?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            node_bundle_get_dir,
            node_bundle_set_dir,
            kns_domain_owner,
            wallet_profiles_list,
            wallet_profile_create_mnemonic,
            wallet_profile_import_mnemonic,
            wallet_profile_import_private_key,
            wallet_profile_delete,
            wallet_profiles_clear_all,
            wallet_unlock,
            wallet_lock,
            wallet_unlocked_username,
            wallet_derive_receive_address,
            wallet_debug_unlocked_material_fingerprint,
            wallet_get_balance,
            wallet_send_kas,
            wallet_status,
            wallet_core_send_file_b64,
            wallet_send_file_b64,
            wallet_send_file_path,
            wallet_receive_file,
            studio_temp_path,
            ffmpeg_transcode,
            open_file,
            reveal_in_folder
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {});
}
