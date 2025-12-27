# Kaspa Audio Transfer Desktop (Tauri)

This is a desktop GUI wrapper around the existing web UI + backend.

## What it does

- Starts the local backend (`kaspa_file_web_backend`) on `http://127.0.0.1:8080`.
- Opens a native window that loads the same interface.

The app supports transferring any file type (not only audio).

## Dev run (Windows)

1. Build the backend once (so the desktop app can start it quickly):

   - `cargo build --manifest-path "web/backend/Cargo.toml"`

2. Run the desktop app:

   - `cargo run --manifest-path "desktop/src-tauri/Cargo.toml"`

## Notes

- If port `8080` is already used, close the other process or change `BACKEND_PORT` support later.
- If the backend exe can’t be found, the app will fallback to running `cargo run` for the backend.
