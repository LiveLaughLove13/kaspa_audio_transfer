use base64::{engine::general_purpose, Engine as _};
use qrcode::render::svg;
use qrcode::QrCode;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::time::Duration;
use tauri::Manager;

mod wallet_kaspa;
mod wallet_vault;

fn force_public_rpc(rpc_url: Option<String>) -> Option<String> {
    match rpc_url.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(s) => {
            let lower = s.to_ascii_lowercase();
            if lower.starts_with("public") {
                Some(s.to_string())
            } else {
                Some("public".to_string())
            }
        }
        None => Some("public".to_string()),
    }
}

fn app_data_root_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("unable to resolve app_data_dir: {e}"))?;
    let new_dir = base.join("KaspaDataTransfer");
    let old_dir = base.join("KaspaAudioTransfer");

    let chosen = if new_dir.exists() || !old_dir.exists() {
        new_dir
    } else {
        old_dir
    };
    std::fs::create_dir_all(&chosen).map_err(|e| e.to_string())?;
    Ok(chosen)
}

fn sanitize_output_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "received.bin".to_string();
    }

    let mut out = String::with_capacity(trimmed.len().min(128));
    for ch in trimmed.chars() {
        if out.len() >= 128 {
            break;
        }
        match ch {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | '.'
            | '-'
            | '_' => out.push(ch),
            _ => out.push('_'),
        }
    }

    let out = out.trim_matches('_').to_string();
    if out.is_empty() {
        "received.bin".to_string()
    } else {
        out
    }
}

const DEFAULT_WALLET_DERIVATION_PATH: &str = "m/44'/111111'/0'/0/0";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WalletTxHistoryItem {
    txid: String,
    timestamp_ms: Option<u64>,
    net_sompi: i64,
    accepted: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
struct RestTxInput {
    previous_outpoint_address: Option<String>,
    previous_outpoint_amount: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
struct RestTxOutput {
    amount: u64,
    script_public_key_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RestTxModel {
    transaction_id: Option<String>,
    block_time: Option<u64>,
    accepting_block_time: Option<u64>,
    is_accepted: Option<bool>,
    inputs: Option<Vec<RestTxInput>>,
    outputs: Option<Vec<RestTxOutput>>,
}

fn kaspa_rest_base_url(network: &str) -> &'static str {
    match network.trim() {
        "mainnet" => "https://api.kaspa.org",
        "testnet" => "https://api-tn10.kaspa.org",
        "devnet" => "https://api.kaspa.org",
        _ => "https://api.kaspa.org",
    }
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_send_file_b64(
    file_b64: String,
    to_address: String,
    amount_kas: f64,
    rpc_url: Option<String>,
    resume_from: Option<String>,
    resume_output_index: Option<u32>,
    from_private_key: Option<String>,
) -> Result<String, String> {
    let rpc_url = force_public_rpc(rpc_url);
    let rpc_url = rpc_url.as_deref();

    let from_private_key = from_private_key
        .as_deref()
        .ok_or_else(|| "missing fromPrivateKey".to_string())?;

    let data_url = file_b64.trim();
    let payload_b64 = data_url
        .split_once(",")
        .map(|(_, b64)| b64)
        .unwrap_or(data_url);

    let bytes = general_purpose::STANDARD
        .decode(payload_b64)
        .map_err(|e| format!("invalid base64: {e}"))?;

    let txid = kaspa_audio_transfer::send_bytes(
        &bytes,
        from_private_key,
        rpc_url,
        resume_from.as_deref(),
        resume_output_index.unwrap_or(1),
        None,
        None,
        &to_address,
        amount_kas,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(txid)
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_receive_file_b64(
    tx_id: String,
    rpc_url: Option<String>,
    start_block_hash: Option<String>,
) -> Result<String, String> {
    let rpc_url = force_public_rpc(rpc_url);
    let rpc_url = rpc_url.as_deref();

    let bytes = kaspa_audio_transfer::receive_bytes(
        tx_id.trim(),
        rpc_url,
        start_block_hash.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(general_purpose::STANDARD.encode(bytes))
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_receive_file_save(
    app: tauri::AppHandle,
    tx_id: String,
    rpc_url: Option<String>,
    start_block_hash: Option<String>,
    output_name: Option<String>,
) -> Result<String, String> {
    let rpc_url = force_public_rpc(rpc_url);
    let rpc_url = rpc_url.as_deref();

    let bytes = kaspa_audio_transfer::receive_bytes(
        tx_id.trim(),
        rpc_url,
        start_block_hash.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut dir = app_data_root_dir(&app)?;
    dir.push("received");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let file_name = sanitize_output_name(output_name.as_deref().unwrap_or("received.bin"));
    dir.push(file_name);
    std::fs::write(&dir, bytes).map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_receive_file_save_downloads(
    app: tauri::AppHandle,
    tx_id: String,
    rpc_url: Option<String>,
    start_block_hash: Option<String>,
    output_name: Option<String>,
) -> Result<String, String> {
    let rpc_url = force_public_rpc(rpc_url);
    let rpc_url = rpc_url.as_deref();

    let bytes = kaspa_audio_transfer::receive_bytes(
        tx_id.trim(),
        rpc_url,
        start_block_hash.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut dir = app
        .path()
        .download_dir()
        .map_err(|e| format!("unable to resolve download_dir: {e}"))?;
    let old = dir.join("KaspaAudioTransfer");
    let new = dir.join("KaspaDataTransfer");
    if new.exists() || !old.exists() {
        dir.push("KaspaDataTransfer");
    } else {
        dir.push("KaspaAudioTransfer");
    }
    dir.push("received");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let file_name = sanitize_output_name(output_name.as_deref().unwrap_or("received.bin"));
    dir.push(file_name);
    std::fs::write(&dir, bytes).map_err(|e| e.to_string())?;

    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profiles_list(app: tauri::AppHandle) -> Result<Vec<wallet_vault::WalletProfileInfo>, String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::list_profiles(&dir)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_create_mnemonic(
    app: tauri::AppHandle,
    username: String,
    password: String,
    word_count: u32,
) -> Result<String, String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::create_profile_mnemonic(&dir, &username, &password, word_count)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_import_mnemonic(
    app: tauri::AppHandle,
    username: String,
    password: String,
    phrase: String,
    mnemonic_password: Option<String>,
) -> Result<(), String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::import_profile_mnemonic(
        &dir,
        &username,
        &password,
        &phrase,
        mnemonic_password.as_deref(),
    )
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_import_private_key(
    app: tauri::AppHandle,
    username: String,
    password: String,
    private_key_hex: String,
) -> Result<(), String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::import_profile_private_key(&dir, &username, &password, &private_key_hex)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profile_delete(app: tauri::AppHandle, username: String) -> Result<(), String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::delete_profile(&dir, &username)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_profiles_clear_all(app: tauri::AppHandle) -> Result<(), String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::clear_all_profiles(&dir)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_unlock(app: tauri::AppHandle, username: String, password: String) -> Result<(), String> {
    let dir = app_data_root_dir(&app)?;
    wallet_vault::unlock_profile(&dir, &username, &password)
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

#[tauri::command(rename_all = "camelCase")]
fn wallet_unlocked_private_key_hex(network: String, derivation_path: Option<String>) -> Result<String, String> {
    let path = derivation_path
        .as_deref()
        .unwrap_or(DEFAULT_WALLET_DERIVATION_PATH);
    wallet_vault::unlocked_private_key_hex_for_path(&network, path)
}

#[tauri::command(rename_all = "camelCase")]
fn wallet_qr_svg(data: String) -> Result<String, String> {
    let d = data.trim();
    if d.is_empty() {
        return Err("data is empty".to_string());
    }

    let code = QrCode::new(d.as_bytes()).map_err(|e| e.to_string())?;
    let out = code
        .render::<svg::Color>()
        .min_dimensions(240, 240)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    Ok(out)
}

#[tauri::command(rename_all = "camelCase")]
async fn rpc_connection_info(
    network: String,
    rpc_url: Option<String>,
) -> Result<wallet_kaspa::RpcConnectionInfo, String> {
    let rpc_url = force_public_rpc(rpc_url);
    wallet_kaspa::rpc_connection_info(&network, rpc_url.as_deref()).await
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_get_balance(
    network: String,
    derivation_path: String,
    rpc_url: Option<String>,
) -> Result<f64, String> {
    let rpc_url = force_public_rpc(rpc_url);
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
    let rpc_url = force_public_rpc(rpc_url);
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
async fn wallet_tx_history(
    network: String,
    address: String,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<WalletTxHistoryItem>, String> {
    let addr = address.trim();
    if addr.is_empty() {
        return Err("address is empty".to_string());
    }
    let limit = limit.unwrap_or(25).min(200).max(1);
    let offset = offset.unwrap_or(0);

    let base = kaspa_rest_base_url(&network);
    let url = format!(
        "{}/addresses/{}/full-transactions?limit={}&offset={}&resolve_previous_outpoints=light",
        base,
        urlencoding::encode(addr),
        limit,
        offset
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("REST client build failed: {e}"))?;

    let txs = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("REST request failed: {e}"))?
        .error_for_status()
        .map_err(|e| format!("REST error: {e}"))?
        .json::<Vec<RestTxModel>>()
        .await
        .map_err(|e| format!("REST json parse failed: {e}"))?;

    let mut out: Vec<WalletTxHistoryItem> = Vec::with_capacity(txs.len());
    for t in txs {
        let txid = match t
            .transaction_id
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(v) => v.to_string(),
            None => continue,
        };

        let mut in_sum: u64 = 0;
        let mut out_sum: u64 = 0;

        if let Some(inputs) = t.inputs.as_ref() {
            for i in inputs {
                if i.previous_outpoint_address.as_deref().map(str::trim) == Some(addr) {
                    if let Some(a) = i.previous_outpoint_amount {
                        in_sum = in_sum.saturating_add(a);
                    }
                }
            }
        }
        if let Some(outputs) = t.outputs.as_ref() {
            for o in outputs {
                if o.script_public_key_address.as_deref().map(str::trim) == Some(addr) {
                    out_sum = out_sum.saturating_add(o.amount);
                }
            }
        }

        let net = out_sum as i128 - in_sum as i128;
        let net_sompi = if net > i64::MAX as i128 {
            i64::MAX
        } else if net < i64::MIN as i128 {
            i64::MIN
        } else {
            net as i64
        };

        let ts = t.accepting_block_time.or(t.block_time).filter(|v| *v > 0);

        out.push(WalletTxHistoryItem {
            txid,
            timestamp_ms: ts,
            net_sompi,
            accepted: t.is_accepted,
        });
    }

    out.sort_by_key(|t| std::cmp::Reverse(t.timestamp_ms.unwrap_or(0)));
    Ok(out)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            wallet_send_file_b64,
            wallet_receive_file_b64,
            wallet_receive_file_save,
            wallet_receive_file_save_downloads,
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
            wallet_unlocked_private_key_hex,
            wallet_qr_svg,
            rpc_connection_info,
            wallet_get_balance,
            wallet_send_kas,
            wallet_tx_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
