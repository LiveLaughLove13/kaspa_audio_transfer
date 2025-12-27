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

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/kaspa_audio_transfer
cd kaspa_audio_transfer

# Build in release mode (recommended)
cargo build --release

# The binary will be available at:
# - Linux/macOS: ./target/release/kaspa_audio_transfer
# - Windows: .\target\release\kaspa_audio_transfer.exe
```

## Usage

### Sending an Audio File

```powershell
# Basic usage
kaspa_audio_transfer send --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --file path\to\your\audio.mp3

cargo run -- send "E:\Eminem - No Love Explicit Version ft. Lil Wayne.mp3" --from-private-key 036aa7c6a91c24af963c4001e599db4fffb4395a2459a80986b9c223320a37cc --to-address kaspa:qq3tqr9f0z6t6zwcrjkk8krwwltazcl0s4gvelvakvqmj9essyq4kaksa3v0m --amount 0.2

# With custom RPC URL (default: grpc://127.0.0.1:16110)
kaspa_audio_transfer send --rpc-url grpc://your.node:port --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --file song.mp3

# Resume a failed send operation
kaspa_audio_transfer send --from-private-key YOUR_PRIVATE_KEY --to-address kaspa:recipient_address --resume-from PREVIOUS_MANIFEST_TXID --resume-output-index 1 --file song.mp3
```

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
