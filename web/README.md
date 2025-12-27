# Web Front-End Design for Kaspa Audio Transfer

This document describes a simple web application that provides a user-friendly interface for sending and (optionally) retrieving files using the `kaspa_audio_transfer` binary.

> **Goal:** Users can drag-and-drop a file in the browser, see a fee estimate, pay, and have their file stored on the Kaspa network without running their own node or CLI.

## 1. High-Level Architecture

- **Frontend (SPA)**
  - Technology: React / Vue / Svelte / similar.
  - Runs in the browser only; no direct access to keys.
  - Interacts with a backend via HTTPS.

- **Backend Service**
  - Technology: can be Rust (e.g., `axum`, `warp`) or Node.js/Express.
  - Responsibilities:
    - Accept uploads.
    - Call the `kaspa_audio_transfer` binary for:
      - `estimate` (to compute approximate file storage cost on Kaspa).
      - `send` (to actually push data on-chain once payment is confirmed).
    - Manage payment requests and job status.

- **Kaspa Node**
  - A fully-synced node you control.
  - Used by the backend and/or the `kaspa_audio_transfer` binary via its RPC interface.

The backend encapsulates your keys and fee logic; the frontend is purely a UI layer.

## 2. User Flows

### 2.1. Upload & Estimate

1. User opens the site.
2. They drag-and-drop a file (or use a file picker) onto the UI.
3. Frontend sends the file to `POST /api/estimate`.
4. Backend:
   - Saves the file temporarily (e.g., to `/tmp/uploads/<job_id>`).
   - Runs something like:
     ```bash
     kaspa_audio_transfer estimate <file_path> \
       --from-private-key <SERVICE_PRIVATE_KEY> \
       --amount <storage_value>
     ```
   - Parses stdout from the `estimate` command to extract:
     - File size.
     - Chunk count and chunk size.
     - Manifest fee, total chunk fees, total estimated network fee.
     - Effective cost per MiB.
   - Calculates a **total price for the user**, e.g.:
     ```
     user_total_price = network_fees + service_fee_margin
     ```
   - Returns this info to the frontend as JSON.
5. Frontend displays:
   - File name and size.
   - Chunking summary.
   - Total price in KAS.
   - A payment address and QR code.

### 2.2. Payment

After the estimate:

1. Backend creates a **payment request** record:
   - `job_id` (UUID).
   - `file_path`.
   - `expected_amount_kas`.
   - `payment_address` (a KAS address you control; could be a dedicated address per job or derived from an HD wallet).
   - `status` (`pending_payment`, `paid`, `sending`, `completed`, `failed`).
2. Frontend displays:
   - The `payment_address` as text + QR.
   - The exact amount to pay in KAS.
3. User pays the specified amount from their own wallet to the provided address.
4. Backend periodically checks for incoming payments:
   - Either by polling (e.g. via RPC calls like `get_utxos_by_addresses`/`get_balances_by_addresses`), or using event subscriptions from the node.
   - When it sees a payment at or above the expected amount (with sufficient confirmations), it marks the job as `paid`.
5. Frontend polls an endpoint like `GET /api/jobs/<job_id>` to see when the status switches from `pending_payment` → `paid` → `sending` → `completed`.

### 2.3. Send to Kaspa

Once a job is marked `paid`:

1. Backend transitions the job to `sending`.
2. Backend calls:
   ```bash
   kaspa_audio_transfer send <file_path> \
     --from-private-key <SERVICE_PRIVATE_KEY> \
     --to-address <user_or_service_address> \
     --amount <storage_value>
   ```
   - If you integrate a **service fee** in on-chain outputs, your internal logic ensures:
     - Manifest transaction includes:
       - Payment to the target address.
       - Service-fee output to your fee address.
       - Change back to your service wallet.
3. Backend captures stdout from `send` to get:
   - Manifest transaction ID (`txid`).
   - Chunk submission logs.
   - Fee summary.
4. Backend updates the job:
   - `status = completed`.
   - `txid = <manifest_txid>`.
   - `block_hash` if known.
5. Frontend shows a success screen:
   - Transaction ID.
   - Optional block hash.
   - Instructions on how to retrieve the file via CLI, e.g.:
     ```
     kaspa_audio_transfer receive <TXID> --output downloaded_file
     ```

### 2.4. Retrieval (Optional Web Feature)

You may also provide a **download** feature via the website:

1. User enters a TXID and desired filename.
2. Frontend calls `POST /api/receive` with the TXID.
3. Backend runs:
   ```bash
   kaspa_audio_transfer receive <TXID> --output -
   ```
   and streams the raw bytes back to the browser.
4. Browser prompts the user to save the file.

This flow is optional; to reduce bandwidth costs, you can choose to only provide instructions for using the CLI to download.

## 3. Directory Layout (Suggested)

A simple structure in the repo:

```text
kaspa_audio_transfer/
  src/                 # Rust CLI code (existing)
  Cargo.toml

  web/
    README.md          # This file (web design)
    backend/           # Future backend implementation
    frontend/          # Future frontend implementation
```

- `web/backend/` might contain a Rust or Node.js project that exposes the HTTP API and shells out to the `kaspa_audio_transfer` binary.
- `web/frontend/` will contain the SPA source (React/Vue/etc.).

## 4. API Sketch

### 4.1. `POST /api/estimate`

**Request** (multipart/form-data):

- `file`: uploaded file.

**Response** (JSON):

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
  "user_total_price_kas": 0.35000000,
  "payment_address": "kaspa:...",
  "payment_qr": "<optional: data URI or URL>",
  "status": "pending_payment"
}
```

### 4.2. `GET /api/jobs/<job_id>`

**Response** (JSON):

```json
{
  "job_id": "<uuid>",
  "status": "pending_payment | paid | sending | completed | failed",
  "txid": "<txid_if_completed>",
  "block_hash": "<block_hash_if_known>",
  "error": null
}
```

### 4.3. `POST /api/receive` (Optional)

**Request** (JSON):

```json
{
  "txid": "<transaction_id>"
}
```

**Response**:

- Binary stream of file data, with appropriate `Content-Type` and `Content-Disposition` headers.

## 5. Security Considerations

- **Key Management**
  - Service private keys must be stored securely (e.g., in environment variables, vaults, or KMS).
  - The frontend must never see or handle private keys.

- **Rate Limiting / Abuse Protection**
  - Add rate limiting on `POST /api/estimate` and `POST /api/receive`.
  - Consider maximum allowed file size per upload.

- **Input Validation**
  - Validate filenames and MIME types.
  - Enforce per-user or per-IP quotas.

- **Job Cleanup**
  - Periodically delete expired temp files for jobs that never get paid.

## 6. Roadmap

1. **Phase 1: Backend Skeleton**
   - Implement `/api/estimate` using the existing `estimate` CLI command.
   - Store job metadata in memory or a lightweight database.

2. **Phase 2: Basic Frontend**
   - Drag-and-drop file UI.
   - Displays estimate and payment instructions.

3. **Phase 3: Automated Payment Detection**
   - Integrate with the Kaspa node to detect incoming payments to your addresses.
   - Automatically transition jobs from `pending_payment` to `paid`.

4. **Phase 4: Sending & Completion UI**
   - Call `send` after payment is detected.
   - Show progress and final TXID in the frontend.

5. **Phase 5 (Optional): Download Feature**
   - Implement `/api/receive` and UI for downloading files from the chain.

This design keeps the web app cleanly separated from the Rust CLI, while still letting you reuse all the existing data-chunking and fee-estimation logic by shelling out to the `kaspa_audio_transfer` binary.

---

## 7. Web Deliverables (Summary)

For the purposes of planning and funding, the concrete website deliverables are:

- **Drag-and-drop upload UI**
  - Single-page interface where users can drop or select a file.

- **Fee estimation view**
  - Shows estimated manifest fee, chunk fees, total network fee, and effective cost per MiB.
  - Displays the user-facing total price (including any service fee, if applicable).

- **Payment & status tracking**
  - Displays a Kaspa payment address and amount.
  - Shows job status transitions: `pending_payment → paid → sending → completed`.

- **Send confirmation screen**
  - Shows the manifest TXID and, when available, the block hash.
  - Provides CLI instructions for retrieval using `kaspa_audio_transfer receive`.

- **(Optional) Download page**
  - Simple form to enter a TXID and download the reconstructed file (backed by `/api/receive`).

These items map directly onto the roadmap phases above and represent the core website "deliverables" promised alongside the CLI tooling.

---

## 8. How to Run (Backend)

These steps assume you are in the root of the repository.

1. **Set environment variables (optional but recommended)**

   Create a `.env` file in `web/backend/` or export variables in your shell:

   ```bash
   # Example .env
   KASPA_RPC_URL=grpc://127.0.0.1:16110
   SERVICE_PRIVATE_KEY=your_service_private_key_here
   SERVICE_RECEIVE_ADDRESS=kaspa:your_receive_address_here
   BACKEND_PORT=8080
   UPLOAD_DIR=tmp/uploads
   ```

2. **Run the Rust backend**

   ```bash
   cd web/backend
   cargo run
   ```

   The server will listen on `http://127.0.0.1:8080` by default. You can verify it is running with:

   ```bash
   curl http://127.0.0.1:8080/health
   ```

3. **Frontend (future)**

   Once implemented, the frontend will live under `web/frontend/` and will talk to this backend via the `/api` endpoints described above. For now, you can interact with the backend directly (e.g., using curl or a tool like Postman) while developing and testing.
