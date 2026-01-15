# Kaspa Data Transfer Desktop (Tauri)

This is the desktop GUI.

## What it does

- Provides a native desktop UI for sending/receiving files.

## Dev run (Windows)

1. Install Tauri CLI v2:

   - `cargo install tauri-cli --version "^2" --locked`

2. Run the desktop app:

   - Run this from `desktop/src-tauri`: `cargo tauri dev`

## Notes

- On Windows, building an `.msi` installer requires WiX Toolset.
