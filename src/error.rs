use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AudioTransferError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Audio decoding error: {0}")]
    AudioDecoding(String),
    
    #[error("Kaspa RPC error: {0}")]
    KaspaRpc(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, AudioTransferError>;
