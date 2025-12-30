use crate::config::Config;
use crate::jobs::{Job, JobStatus, JobStore, SendJob, SendJobStatus, SendJobStore};
use crate::process::{run_estimate, run_receive, run_send, run_tx_accepting_block_hash, EstimateResult, SendResult};
use axum::body::Body;
use axum::extract::{Multipart, Path, Query, State};
use axum::{
    http::{header, StatusCode},
    response::Response,
    Json,
};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use std::time::UNIX_EPOCH;

#[derive(serde::Deserialize)]
pub struct TxAcceptingBlockHashQuery {
    pub tx_id: String,
    pub rpc_url: Option<String>,
    pub start_block_hash: Option<String>,
    pub min_confirmations: Option<u64>,
    pub wait_secs: Option<u64>,
}

#[derive(Serialize)]
pub struct TxAcceptingBlockHashResponse {
    pub tx_id: String,
    pub accepting_block_hash: Option<String>,
}

#[derive(Clone, Default)]
struct SendLockStore {
    inner: Arc<Mutex<HashMap<u64, Arc<Mutex<()>>>>>,
}

#[derive(Clone, Default)]
struct TxAcceptingBlockHashCache {
    inner: Arc<Mutex<HashMap<u64, TxAcceptingBlockHashCacheEntry>>>,
}

#[derive(Clone)]
struct TxAcceptingBlockHashCacheEntry {
    accepting_block_hash: Option<String>,
    stored_at: Instant,
}

#[derive(Clone, Default)]
struct TxAcceptingBlockHashLockStore {
    inner: Arc<Mutex<HashMap<u64, Arc<Mutex<()>>>>>,
}

fn sanitize_upload_filename(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "upload.bin".to_string();
    }

    // Some clients send a full path (e.g. C:\\fakepath\\file.ext). Keep only the last segment.
    let base = trimmed
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(trimmed)
        .trim();

    let mut out = String::with_capacity(base.len());
    for ch in base.chars() {
        // Avoid Windows-invalid filename characters and anything that could create nested paths.
        // https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file
        let ok = matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' | ' ');
        if ok {
            out.push(ch);
        } else {
            out.push('_');
        }
    }

    let cleaned = out.trim_matches([' ', '.']).to_string();
    if cleaned.is_empty() {
        return "upload.bin".to_string();
    }

    // Reasonable limit to prevent extremely long filenames.
    if cleaned.len() > 180 {
        cleaned.chars().take(180).collect()
    } else {
        cleaned
    }
}

#[derive(Clone)]
struct SendQueue {
    inner: Arc<Mutex<HashMap<u64, mpsc::Sender<SendTask>>>>,
}

#[derive(Clone)]
struct SendTask {
    job_id: Uuid,
    file_path: std::path::PathBuf,
    from_private_key: String,
    rpc_url: Option<String>,
    resume_from: Option<String>,
    resume_output_index: u32,
    to_address: String,
    amount_kas: f64,
}

impl SendQueue {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn run_send_streaming(
        send_jobs: &SendJobStore,
        cfg: &Config,
        task: &SendTask,
    ) -> Result<SendResult, String> {
        use std::process::Stdio;
        use tokio::process::Command;

        let amount_arg = format!("{:.8}", task.amount_kas);
        let mut cmd = Command::new(&cfg.kaspa_binary);
        cmd.arg("send")
            .arg(&task.file_path)
            .arg("--from-private-key")
            .arg(&task.from_private_key);

        let rpc_url = task.rpc_url.as_deref().or_else(|| {
            let u = cfg.kaspa_rpc_url.as_str();
            (!u.trim().is_empty()).then_some(u)
        });
        if let Some(u) = rpc_url {
            cmd.arg("--rpc-url").arg(u);
        }
        if let Some(r) = task.resume_from.as_deref() {
            cmd.arg("--resume-from").arg(r);
            cmd.arg("--resume-output-index")
                .arg(task.resume_output_index.to_string());
        }
        cmd.arg("--to-address")
            .arg(&task.to_address)
            .arg("--amount")
            .arg(&amount_arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

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

        let (tx, mut rx) = mpsc::unbounded_channel::<(bool, String)>();
        let tx_out = tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx_out.send((true, line));
            }
        });
        let tx_err = tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx_err.send((false, line));
            }
        });
        drop(tx);

        let mut stderr_buf = String::new();
        let mut tx_id: Option<String> = None;
        while let Some((is_stdout, line)) = rx.recv().await {
            let trimmed = line.trim();

            if is_stdout {
                if let Some(rest) = trimmed.strip_prefix("Chunking file into ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if !parts.is_empty() {
                        if let Ok(total) = parts[0].parse::<u32>() {
                            send_jobs
                                .update(task.job_id, |j| {
                                    j.total_chunks = Some(total);
                                })
                                .await;
                        }
                    }
                }

                if let Some(rest) = trimmed.strip_prefix("Submitted chunk ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if !parts.is_empty() {
                        let frac = parts[0].trim_end_matches(':');
                        if let Some((a, b)) = frac.split_once('/') {
                            if let (Ok(done), Ok(total)) = (a.parse::<u32>(), b.parse::<u32>()) {
                                send_jobs
                                    .update(task.job_id, |j| {
                                        j.total_chunks = j.total_chunks.or(Some(total));
                                        j.submitted_chunks = done;
                                    })
                                    .await;
                            }
                        }
                    }
                }

                if let Some(rest) = trimmed.strip_prefix("Transaction ID:") {
                    let v = rest.trim();
                    if !v.is_empty() {
                        tx_id = Some(v.to_string());
                    }
                }
            } else {
                stderr_buf.push_str(trimmed);
                stderr_buf.push('\n');
            }
        }

        let status = child
            .wait()
            .await
            .map_err(|e| format!("failed waiting for kaspa_audio_transfer: {e}"))?;
        if !status.success() {
            let stderr = stderr_buf.trim();
            return Err(format!(
                "kaspa_audio_transfer send failed with status {}: {}",
                status,
                if stderr.is_empty() { "<no stderr>" } else { stderr }
            ));
        }

        let tx_id = tx_id.ok_or_else(|| {
            "kaspa_audio_transfer send succeeded but no Transaction ID found in output".to_string()
        })?;
        Ok(SendResult { tx_id })
    }

    async fn enqueue(&self, wallet_fingerprint: u64, task: SendTask, state: AppState) {
        let mut map = self.inner.lock().await;
        let tx = if let Some(existing) = map.get(&wallet_fingerprint) {
            existing.clone()
        } else {
            let (tx, mut rx) = mpsc::channel::<SendTask>(32);
            map.insert(wallet_fingerprint, tx.clone());
            let send_jobs = state.send_jobs.clone();
            let cfg = state.config.clone();
            tokio::spawn(async move {
                let mut last_txid: Option<String> = None;
                while let Some(task) = rx.recv().await {
                    let mut task = task;
                    let job_id = task.job_id;
                    send_jobs
                        .update(job_id, |j| {
                            j.status = SendJobStatus::Running;
                            j.error = None;
                            j.submitted_chunks = 0;
                        })
                        .await;

                    if task.resume_from.is_none() {
                        if let Some(prev) = last_txid.as_deref() {
                            task.resume_from = Some(prev.to_string());
                            task.resume_output_index = 1;
                        }
                    }

                    let mut result: Result<SendResult, String> = Err("not started".to_string());
                    let mut hops: u32 = 0;
                    let max_hops: u32 = 4;
                    while hops < max_hops {
                        result = SendQueue::run_send_streaming(&send_jobs, &cfg, &task).await;
                        let Err(e) = &result else { break; };

                        if !is_kaspa_rejected_or_mempool_error(e) {
                            break;
                        }

                        if is_resume_outpoint_missing(e) {
                            if task.resume_from.is_some() {
                                if task.resume_output_index == 1 {
                                    task.resume_output_index = 0;
                                    hops += 1;
                                    continue;
                                }
                                task.resume_from = None;
                                task.resume_output_index = 1;
                                hops += 1;
                                continue;
                            }
                        }

                        let Some(conflict) = extract_mempool_conflict_txid(e) else {
                            break;
                        };

                        if task.resume_from.as_deref() == Some(conflict.as_str()) {
                            if is_resume_outpoint_missing(e) {
                                if task.resume_output_index == 1 {
                                    task.resume_output_index = 0;
                                    hops += 1;
                                    continue;
                                }
                            }
                            break;
                        }

                        task.resume_from = Some(conflict);
                        task.resume_output_index = 1;
                        send_jobs
                            .update(job_id, |j| {
                                j.error = None;
                            })
                            .await;
                        hops += 1;
                    }

                    match result {
                        Ok(SendResult { tx_id }) => {
                            let tx_id_for_store = tx_id.clone();
                            send_jobs
                                .update(job_id, |j| {
                                    j.status = SendJobStatus::Succeeded;
                                    j.txid = Some(tx_id_for_store);
                                    j.error = None;
                                })
                                .await;

                            last_txid = Some(tx_id);
                        }
                        Err(e) => {
                            let blocked = is_kaspa_rejected_or_mempool_error(&e);
                            send_jobs
                                .update(job_id, |j| {
                                    j.status = if blocked {
                                        SendJobStatus::Blocked
                                    } else {
                                        SendJobStatus::Failed
                                    };
                                    j.error = Some(e);
                                })
                                .await;
                        }
                    }
                }
            });

            tx
        };

        let _ = tx.send(task).await;
    }
}

impl SendLockStore {
    async fn lock_for_key(&self, key: &str) -> Arc<Mutex<()>> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let fingerprint = hasher.finish();

        let mut map = self.inner.lock().await;
        if let Some(lock) = map.get(&fingerprint) {
            return lock.clone();
        }

        let lock = Arc::new(Mutex::new(()));
        map.insert(fingerprint, lock.clone());
        lock
    }
}

impl TxAcceptingBlockHashLockStore {
    async fn lock_for_fingerprint(&self, fingerprint: u64) -> Arc<Mutex<()>> {
        let mut map = self.inner.lock().await;
        if let Some(lock) = map.get(&fingerprint) {
            return lock.clone();
        }

        let lock = Arc::new(Mutex::new(()));
        map.insert(fingerprint, lock.clone());
        lock
    }
}

fn is_kaspa_rejected_or_mempool_error(msg: &str) -> bool {
    let m = msg.to_ascii_lowercase();
    m.contains("rejected transaction")
        || m.contains("already spent")
        || m.contains("in the mempool")
        || m.contains("mempool")
}

fn extract_mempool_conflict_txid(msg: &str) -> Option<String> {
    // Example: "already spent by transaction <64hex> in the mempool"
    let re = regex::Regex::new(r"already spent by transaction\s+([0-9a-fA-F]{64})\s+in the mempool").ok()?;
    re.captures(msg)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

fn is_resume_outpoint_missing(msg: &str) -> bool {
    let m = msg.to_ascii_lowercase();
    m.contains("resume_from outpoint") && m.contains("not found")
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub jobs: JobStore,
    pub send_jobs: SendJobStore,
    send_locks: SendLockStore,
    send_queue: SendQueue,
    tx_accepting_block_hash_cache: TxAcceptingBlockHashCache,
    tx_accepting_block_hash_locks: TxAcceptingBlockHashLockStore,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            jobs: JobStore::new(),
            send_jobs: SendJobStore::new(),
            send_locks: SendLockStore::default(),
            send_queue: SendQueue::new(),
            tx_accepting_block_hash_cache: TxAcceptingBlockHashCache::default(),
            tx_accepting_block_hash_locks: TxAcceptingBlockHashLockStore::default(),
        }
    }
}

#[derive(Serialize)]
pub struct EstimateResponse {
    pub job_id: Uuid,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub chunk_count: u32,
    pub chunk_size: u32,
    pub manifest_fee_kas: f64,
    pub chunk_fees_kas: f64,
    pub total_network_fee_kas: f64,
    pub effective_cost_per_mib_kas: Option<f64>,
    pub expected_amount_kas: f64,
    pub payment_address: String,
    pub status: JobStatus,
}

#[derive(Serialize)]
pub struct JobStatusResponse {
    pub job_id: Uuid,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub expected_amount_kas: f64,
    pub payment_address: String,
    pub status: JobStatus,
    pub txid: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct SendResponse {
    pub tx_id: String,
}

#[derive(Serialize)]
pub struct SendJobStatusResponse {
    pub job_id: Uuid,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub to_address: String,
    pub amount_kas: f64,
    pub status: SendJobStatus,
    pub total_chunks: Option<u32>,
    pub submitted_chunks: u32,
    pub txid: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ReceiveRequest {
    pub tx_id: String,
    pub rpc_url: Option<String>,
    pub start_block_hash: Option<String>,
    pub output_name: Option<String>,
}

#[derive(Serialize)]
pub struct LibraryFile {
    pub name: String,
    pub size_bytes: u64,
    pub modified_unix_ms: Option<i64>,
    pub path: String,
}

#[derive(Serialize)]
pub struct LibraryResponse {
    pub base_dir: String,
    pub files: Vec<LibraryFile>,
}

pub async fn estimate_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<EstimateResponse>, (StatusCode, String)> {
    let mut file_name = String::from("upload.bin");
    let mut file_size_bytes: u64 = 0;
    let mut payer_address: Option<String> = None;
    let mut from_private_key: Option<String> = None;
    let mut rpc_url: Option<String> = None;
    let mut storage_amount_kas: f64 = 0.0;

    while let Ok(Some(field)) = multipart.next_field().await {
        let Some(name) = field.name().map(|s| s.to_string()) else {
            continue;
        };

        if name == "payer_address" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("payer_address read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                payer_address = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "from_private_key" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("from_private_key read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                from_private_key = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "rpc_url" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("rpc_url read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                rpc_url = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "amount" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("amount read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                storage_amount_kas = trimmed.parse::<f64>().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("invalid amount '{trimmed}': {e}"),
                    )
                })?;
            }
            continue;
        }

        if name == "file" {
            if let Some(fname) = field.file_name() {
                file_name = sanitize_upload_filename(fname);
            }

            let job_id = Uuid::new_v4();
            let upload_dir = &state.config.upload_dir;
            fs::create_dir_all(upload_dir)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("create_dir_all failed: {e}")))?;

            let safe_name = sanitize_upload_filename(&file_name);
            let path = upload_dir.join(format!("job_{}_{}", job_id, safe_name));
            let mut file = File::create(&path)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file create failed: {e}")))?;

            let mut field = field;
            while let Ok(Some(chunk)) = field.chunk().await {
                file_size_bytes += chunk.len() as u64;
                file.write_all(&chunk)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file write failed: {e}")))?;
            }

            let pk = from_private_key
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| {
                    let v = state.config.service_private_key.as_str();
                    (!v.trim().is_empty()).then_some(v)
                })
                .ok_or_else(|| {
                    (
                        StatusCode::BAD_REQUEST,
                        "missing from_private_key (or set SERVICE_PRIVATE_KEY on backend)".to_string(),
                    )
                })?;

            let lock = state.send_locks.lock_for_key(pk).await;
            let _guard = lock.lock().await;

            let estimate: EstimateResult = run_estimate(
                &path,
                pk,
                rpc_url.as_deref().or_else(|| {
                    let u = state.config.kaspa_rpc_url.as_str();
                    (!u.trim().is_empty()).then_some(u)
                }),
                storage_amount_kas,
                &state.config.kaspa_binary,
            )
            .await
            .map_err(|e| {
                if is_kaspa_rejected_or_mempool_error(&e) {
                    (StatusCode::CONFLICT, e)
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, e)
                }
            })?;

            // For now, expected_amount_kas is equal to network fees (no extra margin).
            let expected_amount_kas = estimate.total_network_fee_kas + storage_amount_kas;
            let payment_address = state.config.service_receive_address.clone();

            let job = Job {
                id: job_id,
                file_path: path,
                file_name: file_name.clone(),
                file_size_bytes,
                expected_amount_kas,
                payment_address: payment_address.clone(),
                payer_address,
                status: JobStatus::PendingPayment,
                txid: None,
                block_hash: None,
                error: None,
            };

            state.jobs.insert(job_id, job).await;

            let resp = EstimateResponse {
                job_id,
                file_name,
                file_size_bytes,
                chunk_count: estimate.chunk_count,
                chunk_size: estimate.chunk_size,
                manifest_fee_kas: estimate.manifest_fee_kas,
                chunk_fees_kas: estimate.chunk_fees_kas,
                total_network_fee_kas: estimate.total_network_fee_kas,
                effective_cost_per_mib_kas: estimate.effective_cost_per_mib_kas,
                expected_amount_kas,
                payment_address,
                status: JobStatus::PendingPayment,
            };

            return Ok(Json(resp));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        "no 'file' field found in multipart form".to_string(),
    ))
}

pub async fn job_status_handler(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<JobStatusResponse>, (StatusCode, String)> {
    let job = state
        .jobs
        .get(job_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("job {job_id} not found")))?;

    let resp = JobStatusResponse {
        job_id: job.id,
        file_name: job.file_name,
        file_size_bytes: job.file_size_bytes,
        expected_amount_kas: job.expected_amount_kas,
        payment_address: job.payment_address,
        status: job.status,
        txid: job.txid,
        block_hash: job.block_hash,
        error: job.error,
    };

    Ok(Json(resp))
}

pub async fn send_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SendResponse>, (StatusCode, String)> {
    let mut file_name = String::from("upload.bin");

    let mut from_private_key: Option<String> = None;
    let mut rpc_url: Option<String> = None;
    let mut resume_from: Option<String> = None;
    let mut resume_output_index: u32 = 1;
    let mut to_address: Option<String> = None;
    let mut amount_kas: Option<f64> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let Some(name) = field.name().map(|s| s.to_string()) else {
            continue;
        };

        if name == "from_private_key" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("from_private_key read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                from_private_key = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "rpc_url" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("rpc_url read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                rpc_url = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "resume_from" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("resume_from read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                resume_from = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "resume_output_index" {
            let text = field
                .text()
                .await
                .map_err(|e| {
                    (StatusCode::BAD_REQUEST, format!("resume_output_index read failed: {e}"))
                })?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                resume_output_index = trimmed.parse::<u32>().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("invalid resume_output_index '{trimmed}': {e}"),
                    )
                })?;
            }
            continue;
        }

        if name == "to_address" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("to_address read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                to_address = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "amount" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("amount read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                let v = trimmed.parse::<f64>().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("invalid amount '{trimmed}': {e}"),
                    )
                })?;
                amount_kas = Some(v);
            }
            continue;
        }

        if name == "file" {
            if let Some(fname) = field.file_name() {
                file_name = sanitize_upload_filename(fname);
            }

            let job_id = Uuid::new_v4();
            let upload_dir = &state.config.upload_dir;
            fs::create_dir_all(upload_dir)
                .await
                .map_err(|e| {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("create_dir_all failed: {e}"))
                })?;

            let safe_name = sanitize_upload_filename(&file_name);
            let path = upload_dir.join(format!("send_{}_{}", job_id, safe_name));
            let mut file = File::create(&path)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file create failed: {e}")))?;

            let mut field = field;
            while let Ok(Some(chunk)) = field.chunk().await {
                file.write_all(&chunk)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file write failed: {e}")))?;
            }

            let pk = from_private_key
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| (StatusCode::BAD_REQUEST, "missing from_private_key".to_string()))?;

            let lock = state.send_locks.lock_for_key(pk).await;
            let _guard = lock.lock().await;

            let to = to_address
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| (StatusCode::BAD_REQUEST, "missing to_address".to_string()))?;

            let amount_kas = amount_kas.unwrap_or(0.0);
            if !amount_kas.is_finite() || amount_kas < 0.0 {
                return Err((StatusCode::BAD_REQUEST, "invalid amount".to_string()));
            }

            let SendResult { tx_id } = run_send(
                &path,
                pk,
                rpc_url.as_deref().or_else(|| {
                    let u = state.config.kaspa_rpc_url.as_str();
                    (!u.trim().is_empty()).then_some(u)
                }),
                resume_from.as_deref(),
                resume_output_index,
                to,
                amount_kas,
                &state.config.kaspa_binary,
            )
            .await
            .map_err(|e| {
                if is_kaspa_rejected_or_mempool_error(&e) {
                    (StatusCode::CONFLICT, e)
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, e)
                }
            })?;

            return Ok(Json(SendResponse { tx_id }));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        "no 'file' field found in multipart form".to_string(),
    ))
}

pub async fn send_async_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SendJobStatusResponse>, (StatusCode, String)> {
    let mut file_name = String::from("upload.bin");
    let mut file_size_bytes: u64 = 0;

    let mut from_private_key: Option<String> = None;
    let mut rpc_url: Option<String> = None;
    let mut resume_from: Option<String> = None;
    let mut resume_output_index: u32 = 1;
    let mut to_address: Option<String> = None;
    let mut amount_kas: Option<f64> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let Some(name) = field.name().map(|s| s.to_string()) else {
            continue;
        };

        if name == "from_private_key" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("from_private_key read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                from_private_key = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "rpc_url" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("rpc_url read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                rpc_url = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "resume_from" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("resume_from read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                resume_from = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "resume_output_index" {
            let text = field.text().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("resume_output_index read failed: {e}"),
                )
            })?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                resume_output_index = trimmed.parse::<u32>().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("invalid resume_output_index '{trimmed}': {e}"),
                    )
                })?;
            }
            continue;
        }

        if name == "to_address" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("to_address read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                to_address = Some(trimmed.to_string());
            }
            continue;
        }

        if name == "amount" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("amount read failed: {e}")))?;
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                let v = trimmed.parse::<f64>().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("invalid amount '{trimmed}': {e}"),
                    )
                })?;
                amount_kas = Some(v);
            }
            continue;
        }

        if name == "file" {
            if let Some(fname) = field.file_name() {
                file_name = sanitize_upload_filename(fname);
            }

            let upload_dir = &state.config.upload_dir;
            fs::create_dir_all(upload_dir)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("create_dir_all failed: {e}")))?;

            let job_id = Uuid::new_v4();
            let safe_name = sanitize_upload_filename(&file_name);
            let path = upload_dir.join(format!("sendjob_{}_{}", job_id, safe_name));
            let mut file = File::create(&path)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file create failed: {e}")))?;

            let mut field = field;
            while let Ok(Some(chunk)) = field.chunk().await {
                file_size_bytes += chunk.len() as u64;
                file.write_all(&chunk)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("file write failed: {e}")))?;
            }

            let pk = from_private_key
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| (StatusCode::BAD_REQUEST, "missing from_private_key".to_string()))?;

            let mut hasher = DefaultHasher::new();
            pk.hash(&mut hasher);
            let wallet_fingerprint = hasher.finish();

            let to = to_address
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| (StatusCode::BAD_REQUEST, "missing to_address".to_string()))?;

            let amount_kas = amount_kas.unwrap_or(0.0);
            if !amount_kas.is_finite() || amount_kas < 0.0 {
                return Err((StatusCode::BAD_REQUEST, "invalid amount".to_string()));
            }

            let job = SendJob {
                id: job_id,
                file_name: file_name.clone(),
                file_size_bytes,
                to_address: to.to_string(),
                amount_kas,
                status: SendJobStatus::Queued,
                total_chunks: None,
                submitted_chunks: 0,
                txid: None,
                error: None,
            };
            state.send_jobs.insert(job_id, job).await;

            let task = SendTask {
                job_id,
                file_path: path,
                from_private_key: pk.to_string(),
                rpc_url,
                resume_from,
                resume_output_index,
                to_address: to.to_string(),
                amount_kas,
            };

            state
                .send_queue
                .enqueue(wallet_fingerprint, task, state.clone())
                .await;

            return Ok(Json(SendJobStatusResponse {
                job_id,
                file_name,
                file_size_bytes,
                to_address: to.to_string(),
                amount_kas,
                status: SendJobStatus::Queued,
                total_chunks: None,
                submitted_chunks: 0,
                txid: None,
                error: None,
            }));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        "no 'file' field found in multipart form".to_string(),
    ))
}

pub async fn send_job_status_handler(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<SendJobStatusResponse>, (StatusCode, String)> {
    let job = state
        .send_jobs
        .get(job_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("send job {job_id} not found")))?;

    Ok(Json(SendJobStatusResponse {
        job_id: job.id,
        file_name: job.file_name,
        file_size_bytes: job.file_size_bytes,
        to_address: job.to_address,
        amount_kas: job.amount_kas,
        status: job.status,
        total_chunks: job.total_chunks,
        submitted_chunks: job.submitted_chunks,
        txid: job.txid,
        error: job.error,
    }))
}

pub async fn receive_handler(
    State(state): State<AppState>,
    Json(req): Json<ReceiveRequest>,
) -> Result<Response, (StatusCode, String)> {
    receive_impl(state, req).await
}

pub async fn tx_accepting_block_hash_handler(
    State(state): State<AppState>,
    Query(q): Query<TxAcceptingBlockHashQuery>,
) -> Result<Json<TxAcceptingBlockHashResponse>, (StatusCode, String)> {
    let tx_id = q.tx_id.trim();
    if tx_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "tx_id is required".to_string()));
    }

    let rpc_url = q
        .rpc_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            let u = state.config.kaspa_rpc_url.as_str();
            (!u.trim().is_empty()).then_some(u)
        });

    let start_block_hash = q
        .start_block_hash
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    // Match CLI send --print-start-block-hash defaults.
    let min_confirmations = q.min_confirmations.unwrap_or(0);
    let wait_secs = q.wait_secs.unwrap_or(15);

    let mut hasher = DefaultHasher::new();
    tx_id.hash(&mut hasher);
    rpc_url.unwrap_or("").hash(&mut hasher);
    start_block_hash.unwrap_or("").hash(&mut hasher);
    min_confirmations.hash(&mut hasher);
    wait_secs.hash(&mut hasher);
    let fingerprint = hasher.finish();

    let ttl_some = Duration::from_secs(10 * 60);
    let ttl_none = Duration::from_secs(2);

    {
        let cache = state.tx_accepting_block_hash_cache.inner.lock().await;
        if let Some(entry) = cache.get(&fingerprint) {
            let ttl = if entry.accepting_block_hash.is_some() {
                ttl_some
            } else {
                ttl_none
            };
            if entry.stored_at.elapsed() <= ttl {
                return Ok(Json(TxAcceptingBlockHashResponse {
                    tx_id: tx_id.to_string(),
                    accepting_block_hash: entry.accepting_block_hash.clone(),
                }));
            }
        }
    }

    let lock = state
        .tx_accepting_block_hash_locks
        .lock_for_fingerprint(fingerprint)
        .await;
    let _guard = lock.lock().await;

    {
        let cache = state.tx_accepting_block_hash_cache.inner.lock().await;
        if let Some(entry) = cache.get(&fingerprint) {
            let ttl = if entry.accepting_block_hash.is_some() {
                ttl_some
            } else {
                ttl_none
            };
            if entry.stored_at.elapsed() <= ttl {
                return Ok(Json(TxAcceptingBlockHashResponse {
                    tx_id: tx_id.to_string(),
                    accepting_block_hash: entry.accepting_block_hash.clone(),
                }));
            }
        }
    }

    let accepting_block_hash = run_tx_accepting_block_hash(
        tx_id,
        rpc_url,
        start_block_hash,
        min_confirmations,
        wait_secs,
        &state.config.kaspa_binary,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    {
        let mut cache = state.tx_accepting_block_hash_cache.inner.lock().await;
        cache.insert(
            fingerprint,
            TxAcceptingBlockHashCacheEntry {
                accepting_block_hash: accepting_block_hash.clone(),
                stored_at: Instant::now(),
            },
        );
    }

    Ok(Json(TxAcceptingBlockHashResponse {
        tx_id: tx_id.to_string(),
        accepting_block_hash,
    }))
}

#[derive(serde::Deserialize)]
pub struct ReceiveQuery {
    pub tx_id: String,
    pub rpc_url: Option<String>,
    pub start_block_hash: Option<String>,
    pub output_name: Option<String>,
}

pub async fn receive_get_handler(
    State(state): State<AppState>,
    Query(q): Query<ReceiveQuery>,
) -> Result<Response, (StatusCode, String)> {
    let req = ReceiveRequest {
        tx_id: q.tx_id,
        rpc_url: q.rpc_url.and_then(|s| {
            let t = s.trim().to_string();
            (!t.is_empty()).then_some(t)
        }),
        start_block_hash: q.start_block_hash.and_then(|s| {
            let t = s.trim().to_string();
            (!t.is_empty()).then_some(t)
        }),
        output_name: q.output_name.and_then(|s| {
            let t = s.trim().to_string();
            (!t.is_empty()).then_some(t)
        }),
    };
    receive_impl(state, req).await
}

async fn receive_impl(
    state: AppState,
    req: ReceiveRequest,
) -> Result<Response, (StatusCode, String)> {
    let tx_id = req.tx_id.trim();
    if tx_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "tx_id is required".to_string()));
    }

    let upload_dir = &state.config.upload_dir;
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("create_dir_all failed: {e}")))?;

    let safe_txid = tx_id.replace('/', "_").replace('\\', "_");
    let output_path = upload_dir.join(format!("recv_{}.bin", safe_txid));

    run_receive(
        tx_id,
        &output_path,
        req.rpc_url.as_deref().or_else(|| {
            let u = state.config.kaspa_rpc_url.as_str();
            (!u.trim().is_empty()).then_some(u)
        }),
        req.start_block_hash.as_deref(),
        &state.config.kaspa_binary,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let file = tokio::fs::File::open(&output_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("failed to open received file: {e}")))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let out_name = req
        .output_name
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .unwrap_or("received.bin");
    let disposition = format!(
        "attachment; filename=\"{}\"",
        out_name.replace('"', "")
    );

    let mut resp = Response::new(body);
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream"),
    );
    resp.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&disposition)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("bad content-disposition: {e}")))?,
    );

    Ok(resp)
}

pub async fn library_handler(
    State(state): State<AppState>,
) -> Result<Json<LibraryResponse>, (StatusCode, String)> {
    let upload_dir = state.config.upload_dir.clone();
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("create_dir_all failed: {e}")))?;

    let mut entries = fs::read_dir(&upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("read_dir failed: {e}")))?;

    let mut files: Vec<LibraryFile> = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let meta = match entry.metadata().await {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !meta.is_file() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        if name.contains('/') || name.contains('\\') || name.contains("..") {
            continue;
        }

        let modified_unix_ms = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as i64);

        let path = upload_dir.join(&name);
        let abs = fs::canonicalize(&path).await.ok();
        files.push(LibraryFile {
            name,
            size_bytes: meta.len(),
            modified_unix_ms,
            path: abs
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string()),
        });
    }

    files.sort_by(|a, b| b.modified_unix_ms.cmp(&a.modified_unix_ms));

    Ok(Json(LibraryResponse {
        base_dir: upload_dir.to_string_lossy().to_string(),
        files,
    }))
}

pub async fn library_file_handler(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    if name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "name is required".to_string()));
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err((StatusCode::BAD_REQUEST, "invalid name".to_string()));
    }

    let upload_dir = state.config.upload_dir.clone();
    let full = upload_dir.join(&name);

    let meta = fs::metadata(&full)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "file not found".to_string()))?;
    if !meta.is_file() {
        return Err((StatusCode::NOT_FOUND, "file not found".to_string()));
    }

    let file = tokio::fs::File::open(&full)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("failed to open file: {e}")))?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let disposition = format!("attachment; filename=\"{}\"", name.replace('"', ""));
    let mut resp = Response::new(body);
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream"),
    );
    resp.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&disposition)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("bad content-disposition: {e}")))?,
    );

    Ok(resp)
}
