# Rust Web Backend Design for Kaspa File Transfer Suite

This document specifies the design of a Rust-based backend service that powers the drag-and-drop web interface for the `kaspa_audio_transfer` tool.

The backend will:

- Accept file uploads from the browser.
- Call the existing `kaspa_audio_transfer` CLI for `estimate`, `send`, and optionally `receive`.
- Track job status (pending payment → paid → sending → completed/failed).
- Keep all private keys and node access **off** the frontend.

> **Scope:** This is a design document only. It describes how the backend should be structured and behave, so it can be implemented later.

---

## 1. Technology Stack

- **Language:** Rust
- **Web framework:** `axum` (on top of `hyper`)
- **Async runtime:** `tokio`
- **JSON (de)serialization:** `serde`, `serde_json`
- **Configuration:** environment variables (optionally `.env` + `dotenvy`)
- **Process management:** `tokio::process::Command` to shell out to `kaspa_audio_transfer`

The backend will be a single Rust binary that listens on an HTTP port and exposes a small set of REST-like endpoints under `/api`.

---

## 2. Project Layout

Within the existing `web` folder, the backend project will live under `web/backend`:

```text
web/
  backend/
    Cargo.toml
    src/
      main.rs       # App bootstrap, router setup, background tasks
      config.rs     # Load environment variables and constants
      jobs.rs       # Job types, job store, status transitions
      process.rs    # Helpers for calling kaspa_audio_transfer
      routes.rs     # HTTP handlers for /api endpoints
```

### 2.1. Responsibilities per Module

- **`main.rs`**
  - Initialize logging.
  - Load configuration.
  - Construct `axum` router and mount `/api` routes.
  - Initialize shared job store (in-memory) and pass via `Extension` / `State`.
  - Spawn background tasks for payment detection.
  - Start HTTP server on configured port.

- **`config.rs`**
  - Define a `Config` struct with fields like:
    - `kaspa_rpc_url: String`
    - `service_private_key: String`
    - `service_receive_address: String`
    - `backend_port: u16`
    - `upload_dir: PathBuf`
  - Load values from environment variables with defaults where appropriate.

- **`jobs.rs`**
  - Define `JobStatus` enum:
    - `PendingPayment`
    - `Paid`
    - `Sending`
    - `Completed`
    - `Failed { reason: String }`
  - Define `Job` struct with:
    - `id: Uuid`
    - `file_path: PathBuf`
    - `file_name: String`
    - `file_size_bytes: u64`
    - `expected_amount_kas: f64`
    - `payment_address: String`
    - `status: JobStatus`
    - `txid: Option<String>`
    - `block_hash: Option<String>`
    - `created_at: DateTime<Utc>`
  - Define `JobStore` as something like:
    - `Arc<RwLock<HashMap<Uuid, Job>>>`
  - Implement helper methods:
    - `create_job(...) -> Job`
    - `get_job(id) -> Option<Job>`
    - `update_job_status(id, new_status)`

- **`process.rs`**
  - Functions to shell out to `kaspa_audio_transfer`:
    - `run_estimate(file_path: &Path, storage_amount_kas: f64) -> EstimateResult`
    - `run_send(file_path: &Path, storage_amount_kas: f64, to_address: &str) -> SendResult`
    - Optional: `run_receive(txid: &str) -> ReceiveResult` for download.
  - Each function uses `tokio::process::Command` and captures stdout/stderr.
  - Parse CLI output to extract:
    - File size, chunk count, chunk size.
    - Fee breakdown (manifest, chunks, total).
    - TXID and block hash (for `send`).

- **`routes.rs`**
  - Handlers for HTTP routes:
    - `POST /api/estimate`
    - `GET /api/jobs/{id}`
    - `POST /api/send/{id}` or internal triggering after payment
    - Optional: `POST /api/receive`

---

## 3. Configuration and Environment

Backend configuration is driven by environment variables (possibly in a `.env` file):

- `KASPA_RPC_URL`: RPC URL of the Kaspa node, e.g. `grpc://127.0.0.1:16110`.
- `SERVICE_PRIVATE_KEY`: Private key used by the backend to sign `send` transactions.
- `SERVICE_RECEIVE_ADDRESS`: Address where users send KAS payments.
- `BACKEND_PORT`: Port for this HTTP service (e.g., `8080`).
- `UPLOAD_DIR`: Directory for temporary uploaded files (e.g., `tmp/uploads`).

`config.rs` will read these at startup and return a `Config` instance used throughout the app.

---

## 4. API Design

All endpoints live under `/api` and speak JSON (except for file uploads and optional download).

### 4.1. `POST /api/estimate`

**Purpose:**

- Accept a file from the frontend and return a fee estimate plus a job ID.

**Request:**

- `Content-Type: multipart/form-data`
- Part: `file` (the uploaded file)

**Backend steps:**

1. Generate a `job_id` (UUID).
2. Save the file to `UPLOAD_DIR/job_<uuid>.bin`.
3. Call `run_estimate`:
   - Shells out to:
     ```text
     kaspa_audio_transfer estimate <file_path> \
       --from-private-key $SERVICE_PRIVATE_KEY \
       --amount <storage_amount>
     ```
   - Parses output for file size, chunk count, chunk size, manifest fee, chunk fees, total fee, cost per MiB.
4. Compute `expected_amount_kas` for the user:
   - `expected_amount_kas = total_network_fee_kas + service_fee_margin`.
5. Create a `Job` with status `PendingPayment` and store it.

**Response (JSON):**

```json
{
  "job_id": "<uuid>",
  "file_name": "example.mp3",
  "file_size_bytes": 7544718,
  "chunk_count": 306,
  "chunk_size": 24703,
  "manifest_fee_kas": 0.00050043,
  "chunk_fees_kas": 0.30542400,
  "total_network_fee_kas": 0.30592443,
  "effective_cost_per_mib_kas": 0.04012345,
  "expected_amount_kas": 0.35000000,
  "payment_address": "kaspa:...",
  "status": "pending_payment"
}
```

### 4.2. `GET /api/jobs/{id}`

**Purpose:**

- Allow the frontend to poll the status of a job.

**Response (JSON):**

```json
{
  "job_id": "<uuid>",
  "status": "pending_payment | paid | sending | completed | failed",
  "txid": "<txid_if_completed>",
  "block_hash": "<block_hash_if_known>",
  "error": null
}
```

### 4.3. Payment Detection (Background Logic)

- A background task periodically checks the Kaspa node for payments to `SERVICE_RECEIVE_ADDRESS`.
- When a payment matching `expected_amount_kas` (within a tolerance) is observed for a job:
  - Update `status` to `Paid`.
  - Optionally enqueue or directly trigger `send` for that job.

### 4.4. `POST /api/send/{id}`

**Purpose:**

- Trigger the actual on-chain send for a job that has been paid.

**Preconditions:**

- Job exists.
- Job status is `Paid`.

**Backend steps:**

1. Mark job status as `Sending`.
2. Call `run_send` for that job:
   - Shells out to:
     ```text
     kaspa_audio_transfer send <file_path> \
       --from-private-key $SERVICE_PRIVATE_KEY \
       --to-address <target_address> \
       --amount <storage_amount>
     ```
   - Parses TXID and any fee summary.
3. On success:
   - Update job: `status = Completed`, set `txid`, optionally `block_hash`.
4. On failure:
   - Update job: `status = Failed`, set `error` string.

**Response (JSON):**

```json
{
  "job_id": "<uuid>",
  "status": "completed",
  "txid": "4f1ff5b8...29d0",
  "block_hash": "<optional>",
  "error": null
}
```

### 4.5. Optional `POST /api/receive`

**Purpose:**

- Allow the frontend to request a file by TXID and download it through the backend.

**Request (JSON):**

```json
{
  "txid": "<transaction_id>"
}
```

**Behavior:**

- Backend runs `kaspa_audio_transfer receive <TXID> --output -` and streams response.
- Sets appropriate `Content-Type` and `Content-Disposition` headers.

This endpoint is optional; you may choose to provide only CLI instructions for retrieval to minimize backend bandwidth and complexity.

---

## 5. Safety Considerations

- **Key isolation:**
  - Private keys only exist in backend environment variables or secure stores.
  - The frontend never sees or handles private keys.

- **Rate limiting and size limits:**
  - Enforce a maximum file size for uploads.
  - Rate-limit `POST /api/estimate` and `POST /api/send/{id}`.

- **Temporary storage cleanup:**
  - Implement periodic cleanup of old files in `UPLOAD_DIR` for jobs that never progress beyond `PendingPayment` or that are very old.

- **Error handling:**
  - Surface clear error messages to the frontend without leaking sensitive internal details.
  - Log full errors on the backend for debugging.

---

## 6. Frontend Integration Notes

The frontend (drag-and-drop web UI) will:

1. Upload a file to `POST /api/estimate`.
2. Display the returned estimate, job ID, and payment address.
3. Poll `GET /api/jobs/{id}` to track status.
4. Optionally call `POST /api/send/{id}` or rely on automatic sending once payment is detected.
5. Show the final TXID and block hash to the user and provide CLI instructions:
   ```bash
   kaspa_audio_transfer receive <TXID> --output downloaded_file
   ```

This backend design keeps most complexity in a well-structured Rust service, while the frontend can remain relatively thin and focused on user experience.
