#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
};

use std::io::{self, BufRead};

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use tauri::{Manager, Window};

struct BackendProcess(Mutex<Option<Child>>);

#[derive(Serialize)]
struct WalletStatus {
    unlocked_account_id: Option<String>,
}

#[derive(Clone, Serialize)]
struct KaspaSendProgressEvent {
    submitted_chunks: u32,
    total_chunks: Option<u32>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WalletSendFileResult {
    tx_id: String,
    start_block_hash: Option<String>,
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

fn extract_first_64hex(line: &str) -> Option<String> {
    let t = line.trim();
    if t.len() == 64 && t.bytes().all(|b| b.is_ascii_hexdigit()) {
        Some(t.to_string())
    } else {
        None
    }
}

fn resolve_start_block_hash(
    exe: &Path,
    tx_id: &str,
    rpc_url: Option<&str>,
    min_confirmations: u64,
    wait_secs: u64,
) -> Result<Option<String>, String> {
    let mut cmd = Command::new(exe);
    cmd.arg("tx-accepting-block-hash")
        .arg(tx_id)
        .arg("--min-confirmations")
        .arg(min_confirmations.to_string())
        .arg("--wait-secs")
        .arg(wait_secs.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(u) = rpc_url.map(str::trim).filter(|s| !s.is_empty()) {
        cmd.arg("--rpc-url").arg(u);
    }

    let out = cmd
        .output()
        .map_err(|e| format!("failed to run tx-accepting-block-hash: {e}"))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "tx-accepting-block-hash failed with status {}: {}",
            out.status,
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines() {
        if let Some(h) = extract_first_64hex(line) {
            return Ok(Some(h));
        }
    }
    Ok(None)
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
    let result = wallet_core_send_file_b64_with_start_block_hash(
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
    .await?;

    Ok(result.tx_id)
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_core_send_file_b64_with_start_block_hash(
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
) -> Result<WalletSendFileResult, String> {
    // The UI prefers this command when using wallet-core for key custody.
    // This desktop build currently does not integrate kaspa wallet-core signing.
    // Trigger the UI fallback path unless a private key is explicitly provided.
    if from_private_key.as_deref().map(str::trim).unwrap_or("").is_empty() {
        return Err(
            "wallet_core_send_file_b64 is not available in this build (wallet-core signing not implemented); use legacy wallet_send_file_b64 with fromPrivateKey"
                .to_string(),
        );
    }

    tauri::async_runtime::spawn_blocking(move || {
        let _ = account_id;

        let to_address = to_address.trim();
        if to_address.is_empty() {
            return Err("missing toAddress".to_string());
        }

        let from_private_key = from_private_key
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "missing fromPrivateKey".to_string())?
            .to_string();

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

        let tx_id = txid.ok_or_else(|| "send succeeded but no Transaction ID found in output".to_string())?;

        // Match CLI send --print-start-block-hash defaults.
        let start_block_hash = resolve_start_block_hash(
            &exe,
            &tx_id,
            rpc_url.as_deref(),
            0,
            15,
        )?;

        Ok(WalletSendFileResult {
            tx_id,
            start_block_hash,
        })
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
    let result = wallet_send_file_b64_with_start_block_hash(
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

    .map(|r| r.tx_id)
}

#[tauri::command(rename_all = "camelCase")]
async fn wallet_send_file_b64_with_start_block_hash(
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
) -> Result<WalletSendFileResult, String> {
    wallet_core_send_file_b64_with_start_block_hash(
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

fn candidate_backend_paths() -> Vec<PathBuf> {
    let exe_name = if cfg!(windows) {
        "kaspa_file_web_backend.exe"
    } else {
        "kaspa_file_web_backend"
    };

    // This file lives at: desktop/src-tauri/src/main.rs
    // We want to locate: web/backend/target/{debug,release}/kaspa_file_web_backend(.exe)
    let mut out = Vec::new();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent() // desktop
        .and_then(|p| p.parent()) // repo root
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.clone());

    out.push(repo_root.join("web").join("backend").join("target").join("debug").join(exe_name));
    out.push(repo_root.join("web").join("backend").join("target").join("release").join(exe_name));

    // If user already built backend elsewhere.
    if let Ok(v) = std::env::var("KASPA_WEB_BACKEND_EXE") {
        let p = PathBuf::from(v);
        out.insert(0, p);
    }

    out
}

fn find_backend_exe() -> Option<PathBuf> {
    candidate_backend_paths()
        .into_iter()
        .find(|p| std::fs::metadata(p).is_ok())
}

fn spawn_backend() -> io::Result<Child> {
    // In dev/debug builds, prefer `cargo run` so the backend always matches the current source.
    // This avoids confusing stale-binary issues (e.g. missing newer API routes).
    let prefer_cargo_run = cfg!(debug_assertions) && std::env::var("KASPA_WEB_BACKEND_EXE").is_err();
    let inherit_stdio = cfg!(debug_assertions);

    if !prefer_cargo_run {
        // Prefer running an existing compiled backend binary.
        if let Some(exe) = find_backend_exe() {
            let mut cmd = Command::new(exe);
            cmd.env("BACKEND_PORT", "8080")
                .stdin(Stdio::null());

            if inherit_stdio {
                cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            } else {
                cmd.stdout(Stdio::null()).stderr(Stdio::null());
            }

            return cmd.spawn();
        }
    }

    // Fallback to cargo run (dev convenience).
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.clone());

    let backend_manifest = repo_root.join("web").join("backend").join("Cargo.toml");

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--manifest-path")
        .arg(backend_manifest)
        .env("BACKEND_PORT", "8080")
        .stdin(Stdio::null());

    if inherit_stdio {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    cmd.spawn().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to spawn backend via cargo: {e}")))
}

fn main() {
    tauri::Builder::default()
        .manage(BackendProcess(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            wallet_status,
            wallet_core_send_file_b64,
            wallet_core_send_file_b64_with_start_block_hash,
            wallet_send_file_b64,
            wallet_send_file_b64_with_start_block_hash,
            open_file,
            reveal_in_folder
        ])
        .setup(|app| {
            let backend = spawn_backend().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let state = app.state::<BackendProcess>();
            *state.0.lock().unwrap() = Some(backend);
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();

                let state = app_handle.state::<BackendProcess>();
                if let Some(mut child) = state.0.lock().unwrap().take() {
                    let _ = child.kill();
                }

                std::process::exit(0);
            }
        });
}
