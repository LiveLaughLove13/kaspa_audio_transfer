use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode an audio file and send it over the Kaspa network
    Send {
        /// Path to the audio file to send
        input_file: String,

        /// Sender private key (32-byte hex, Schnorr)
        #[arg(long)]
        from_private_key: String,
        
        /// Kaspa node RPC URL
        #[arg(long, default_value = "grpc://127.0.0.1:16110")]
        rpc_url: String,

        /// Resume by spending an output of an existing transaction already in the mempool (e.g. a previously submitted manifest tx)
        #[arg(long)]
        resume_from: Option<String>,

        /// Which output index of --resume-from to spend (default: 1 = change output)
        #[arg(long, default_value_t = 1)]
        resume_output_index: u32,
        
        /// Recipient's Kaspa address
        #[arg(long)]
        to_address: String,
        
        /// Amount of KAS to send (in KAS)
        #[arg(long, default_value_t = 0.0)]
        amount: f64,

        /// After sending, attempt to print the accepting block hash (useful as receive --start-block-hash).
        #[arg(long, default_value_t = false)]
        print_start_block_hash: bool,

        /// Minimum confirmations required when resolving accepting block hash.
        #[arg(long, default_value_t = 1)]
        start_block_hash_min_confirmations: u64,

        /// Max seconds to wait for accepting block hash when --print-start-block-hash is set.
        #[arg(long, default_value_t = 120)]
        start_block_hash_timeout_secs: u64,
    },
    
    /// Receive and decode an audio file from the Kaspa network
    Receive {
        /// Transaction ID containing the audio data
        tx_id: String,
        
        /// Output file path (default: output.mp3)
        #[arg(short, long, default_value = "output.mp3")]
        output: String,
        
        /// Kaspa node RPC URL
        #[arg(long, default_value = "grpc://127.0.0.1:16110")]
        rpc_url: String,

        /// Optional block hash to start scanning from (speeds up lookup when known)
        #[arg(long)]
        start_block_hash: Option<String>,
    },

    /// Estimate the Kaspa network fees required to store a file
    Estimate {
        /// Path to the file to estimate fees for
        input_file: String,

        /// Sender private key (32-byte hex, Schnorr). Used only to model script type for fee estimation.
        #[arg(long)]
        from_private_key: String,

        /// Kaspa node RPC URL
        #[arg(long, default_value = "grpc://127.0.0.1:16110")]
        rpc_url: String,

        /// Amount of KAS to send alongside the data (in KAS). Defaults to 0 (data-only storage).
        #[arg(long, default_value_t = 0.0)]
        amount: f64,
    },

    /// Resolve the accepting block hash for a transaction id (useful for receive --start-block-hash)
    TxAcceptingBlockHash {
        /// Transaction ID to resolve
        tx_id: String,

        /// Kaspa node RPC URL
        #[arg(long, default_value = "grpc://127.0.0.1:16110")]
        rpc_url: String,

        /// Optional block hash to start scanning from (speeds up lookup when known)
        #[arg(long)]
        start_block_hash: Option<String>,

        /// Minimum confirmations required
        #[arg(long, default_value_t = 1)]
        min_confirmations: u64,

        /// Wait up to this many seconds for acceptance (0 = no wait)
        #[arg(long, default_value_t = 0)]
        wait_secs: u64,
    },
}
