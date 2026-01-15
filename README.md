# Kaspa Data Transfer

Store and retrieve files on the Kaspa network.

This repository contains:

- **CLI**: `kaspa_data_transfer` (Rust)
- **Desktop app**: `desktop/src-tauri` (Tauri)
- **Android app**: `mobile/app/src-tauri` (Tauri Mobile)

## Features

- Send files (audio, video, documents, images, etc.) to the Kaspa blockchain
- Retrieve files using transaction ID
- Resume interrupted transfers
- Automatic chunking of large files

## Prerequisites

### Required (all targets)

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Git
- Access to a Kaspa node RPC endpoint

### Desktop (Tauri)

- **Tauri CLI v2** (installed via Cargo)
- On Windows, **WiX Toolset** is required for building an `.msi` bundle

### Android (Tauri Mobile)

- Android Studio (or Android SDK + command-line tools)
- JDK (Android Studio bundled JBR/JDK is fine)
- Android NDK installed/configured (as required by Tauri Android builds)

## Kaspa node (local) 

Download `kaspad` from the official Rusty Kaspa releases:

- https://github.com/kaspanet/rusty-kaspa/releases

```bash
# Mainnet (with UTXO index)
kaspad --utxoindex
```

```bash
# Testnet-10 (with UTXO index)
kaspad --testnet --netsuffix=10 --utxoindex
```

Testnet coins:

- https://faucet-tn10.kaspanet.io/

## Get the source

```bash
git clone https://github.com/LiveLaughLove13/kaspa_audio_transfer && cd kaspa_audio_transfer
```

```powershell
git clone https://github.com/LiveLaughLove13/kaspa_audio_transfer; cd kaspa_audio_transfer
```

## CLI (kaspa_data_transfer)

### Build (release)

```bash
cargo build --release --bin kaspa_data_transfer
```

### Run

```bash
cargo run --release --bin kaspa_data_transfer -- --help
```

### Common commands

```powershell
kaspa_data_transfer send "path\\to\\file" --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:RECIPIENT_ADDRESS --amount 0
```

```powershell
kaspa_data_transfer receive TX_ID --output output.bin
```

```powershell
kaspa_data_transfer estimate "path\\to\\file" --from-private-key YOUR_PRIVATE_KEY
```

```powershell
kaspa_data_transfer tx-accepting-block-hash TX_ID
```

### RPC URL

By default the CLI uses:

- Mainnet: `grpc://127.0.0.1:16110`

Override with `--rpc-url`:

```powershell
kaspa_data_transfer send "path\\to\\file" --rpc-url grpc://127.0.0.1:16210 --from-private-key YOUR_PRIVATE_KEY --to-address kaspatest:RECIPIENT_ADDRESS --amount 0
```

## Desktop app (Tauri)

### Install Tauri CLI (v2)

```bash
cargo install tauri-cli --version "^2" --locked
```

### Dev run

Run this from `desktop/src-tauri`:

```bash
cargo tauri dev
```

### Build bundle

Run this from `desktop/src-tauri`:

```bash
cargo tauri build --bundles msi
```

Notes:

- The desktop app bundles the `kaspa_data_transfer` helper binary into the installer.
- On Windows, building the `.msi` requires WiX Toolset.

## Android app (Tauri Mobile)

### Install Tauri CLI (v2)

```bash
cargo install tauri-cli --version "^2" --locked
```

### Build a tester APK (arm64 debug)

Run this from `mobile/app`:

```powershell
cargo tauri android build --debug --target aarch64 --split-per-abi -c .\src-tauri\tauri.conf.json
```

APK output path:

```text
mobile\app\src-tauri\gen\android\app\build\outputs\apk\arm64\debug\app-arm64-debug.apk
```

## How it works (high level)

- **Send**
  - The file is chunked.
  - A manifest transaction is created with metadata.
  - Chunk transactions are submitted and linked to the manifest.
- **Receive**
  - The manifest is read from the provided TX ID.
  - Chunk transactions are discovered and reassembled into the original file.

## Troubleshooting

- **Kaspa node not synced / RPC errors**
  - Ensure your node is fully synced and the gRPC port is reachable.
- **Desktop MSI build fails on Windows**
  - Install WiX Toolset and try again.
- **Android build fails**
  - Confirm Android Studio SDK/NDK are installed and your JDK is configured.

## License

MIT
