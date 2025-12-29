# Kaspa Audio Transfer

A command-line tool to store and retrieve files on the Kaspa blockchain (works for any file type, not just audio).

## Features

- Send files (audio, video, documents, images, etc.) to the Kaspa blockchain
- Retrieve files using transaction ID
- Resume interrupted transfers
- Automatic chunking of large files

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Access to a Kaspa node (local or remote)

### Setup Local Node

```bash
# Mainnet
# Standard setup
kaspad --utxoindex

# Testnet-10
# For testing without real funds
kaspad --testnet --netsuffix=10 --utxoinde

# Get Testnet KAS
# Visit: https://faucet-tn10.kaspanet.io/
```

## Installation

```bash
# Clone the repository
git clone https://github.com/LiveLaughLove13/kaspa_audio_transfer
cd kaspa_audio_transfer

# Build in release mode (recommended)
cargo build --release --bin kaspa_audio_transfer
cargo run --manifest-path desktop/src-tauri/Cargo.toml  For GUI
cargo tauri build

# The binary will be available at:
# - Linux/macOS: ./target/release/kaspa_audio_transfer
# - Windows: .\target\release\kaspa_audio_transfer.exe
```

## Usage

### Sending an Audio File

```powershell
# Basic usage
kaspa_audio_transfer send --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --file path\to\your\audio.mp3

cargo run -- send "E:\Eminem - No Love Explicit Version ft. Lil Wayne.mp3" --from-private-key 1234567891234567891234567891234512345678912345678912345678912345 --to-address kaspa:qq3tqr9f0z6t6zwcrjkk8krwwltazcl0s4gvelvakvqmj9essyq4kaksa3v0m --amount 0.2

# Test download a known upload example
.\kaspa_audio_transfer.exe receive f30623784fd7be906ce7cee1c1f172dd45035e08a2c816402ed89365724e9010 --output kaspa_isKing.mp3 --start-block-hash b8e8f9e94d274c350a59b0f596fcafc6b423b7bdda57e3de9b14bdea5b7ad94b

# With custom RPC URL (default: grpc://127.0.0.1:16110)
kaspa_audio_transfer send --rpc-url grpc://your.node:port --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --file song.mp3

# Resume a failed send operation
kaspa_audio_transfer send --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --resume-from PREVIOUS_MANIFEST_TXID --resume-output-index 1 --file song.mp3
```

## Testnet usage

```powershell
# Basic usage
kaspa_audio_transfer send --rpc-url grpc://127.0.0.1:16210 --from-private-key YOUR_PRIVATE_KEY --to-address kaspatest:recipient_address --file path\to\your\audio.mp3

cargo run -- send "E:\Eminem - No Love Explicit Version ft. Lil Wayne.mp3" --rpc-url grpc://127.0.0.1:16210 --from-private-key 1234567891234567891234567891234512345678912345678912345678912345 --to-address kaspatest:qq3tqr9f0z6t6zwcrjkk8krwwltazcl0s4gvelvakvqmj9essyq4kaksa3v0m --amount 0.2

# Test download (example only, insert real TXID)
.\kaspa_audio_transfer.exe receive f30623784fd7be906ce7cee1c1f172dd45035e08a2c816402ed89365724e9010 --output kaspa_isKing.mp3 --rpc-url grpc://127.0.0.1:16210 --start-block-hash b8e8f9e94d274c350a59b0f596fcafc6b423b7bdda57e3de9b14bdea5b7ad94b 

# Resume a failed send operation
kaspa_audio_transfer send --rpc-url grpc://127.0.0.1:16210 --from-private-key YOUR_PRIVATE_KEY --to-address kaspatest:recipient_address --resume-from PREVIOUS_MANIFEST_TXID --resume-output-index 1 --file song.mp3
```

Blank explanation of flow

if you want to send a file you do
.\kaspa_audio_transfer.exe send "full path here" --from-private-key <a Priv here> --to-address <any kaspa address here> --amount 1 
It will give you the TX, which you then use to get block hash on the mainnet explorer [text](https://explorer.kaspa.org/).
then you do 

.\kaspa_audio_transfer.exe receive <tx it gives at bottom> --output <file.type> --start-block-hash <first block hashof tx> 

if testing on testnet

# if you want to send a file on testnet you do
.\kaspa_audio_transfer.exe send "full path here" --rpc-url grpc://127.0.0.1:16210 --from-private-key <a Priv here> --to-address <any kaspa address here> --amount 1 
It will give you the TX, which you then use to get block hash on the testnet explorer [text](https://explorer-tn10.kaspa.org/).
then you do 

.\kaspa_audio_transfer.exe receive <TX_ID> --output <file> --rpc-url grpc://127.0.0.1:16210 --start-block-hash <first block hash of tx>


### Receiving an Audio File

```powershell
# Basic usage (will scan from pruning point)
kaspa_audio_transfer receive TX_ID --output output.mp3

# Faster retrieval with start block hash
kaspa_audio_transfer receive TX_ID --output received_song.mp3 --start-block-hash BLOCK_HASH

# Example with real values
cargo run -- receive b0c3220031a009c0b8bf71411acab1657ca8680b535bcee704f3e9e80939d6c1 --output my_song.mp3 --start-block-hash eb92329b04ffe0bd70357b365e50309c9daee8cb8751d26933b62da5283840fc
```

## How It Works

1. **Sending**:
   - Splits the audio file into chunks (max ~24KB each)
   - Creates a manifest transaction containing metadata
   - Sends chunk transactions referencing the manifest
   - Each chunk is linked using a unique file ID

2. **Receiving**:
   - Retrieves the manifest transaction using the provided TX ID
   - Scans for chunk transactions using the file ID
   - Reassembles the chunks in order
   - Saves the complete audio file

## Troubleshooting

### Common Issues

1. **RPC Timeouts**:
   - Ensure your Kaspa node is synced and responsive
   - Try increasing the RPC timeout in the code if needed

2. **Missing Chunks**:
   - The tool will retry several times automatically
   - Check if the chunks exist in the mempool or recent blocks

3. **Insufficient Funds**:
   - Sending requires KAS for transaction fees
   - Each chunk requires a small amount of KAS for fees

## License

MIT
