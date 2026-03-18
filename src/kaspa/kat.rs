//! KAT (Kaspa Audio Transfer) manifest and chunk encoding/decoding.

use crate::error::{AudioTransferError, Result};

pub const KAT_MAGIC: &[u8; 4] = b"KAT1";
pub const KAT_TYPE_MANIFEST: u8 = 1;
pub const KAT_TYPE_CHUNK: u8 = 2;
pub const MAX_CHUNK_DATA_SIZE: usize = 24_000;

pub type FileId = [u8; 16];

#[derive(Debug, Clone)]
pub struct KatManifest {
    pub file_id: FileId,
    pub total_size: u64,
    pub chunk_size: u32,
    pub total_chunks: u32,
}

pub fn is_kat_payload(payload: &[u8]) -> bool {
    payload.len() >= 5 && payload[0..4] == *KAT_MAGIC
}

pub fn encode_manifest_payload(manifest: &KatManifest) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 1 + 16 + 8 + 4 + 4);
    out.extend_from_slice(KAT_MAGIC);
    out.push(KAT_TYPE_MANIFEST);
    out.extend_from_slice(&manifest.file_id);
    out.extend_from_slice(&manifest.total_size.to_le_bytes());
    out.extend_from_slice(&manifest.chunk_size.to_le_bytes());
    out.extend_from_slice(&manifest.total_chunks.to_le_bytes());
    out
}

pub fn decode_manifest_payload(payload: &[u8]) -> Result<KatManifest> {
    if payload.len() < 37 {
        return Err(AudioTransferError::InvalidInput(
            "Invalid manifest payload".to_string(),
        ));
    }
    if payload[0..4] != *KAT_MAGIC || payload[4] != KAT_TYPE_MANIFEST {
        return Err(AudioTransferError::InvalidInput(
            "Invalid manifest payload".to_string(),
        ));
    }
    let mut file_id: FileId = [0u8; 16];
    file_id.copy_from_slice(&payload[5..21]);
    let total_size = u64::from_le_bytes(payload[21..29].try_into().unwrap());
    let chunk_size = u32::from_le_bytes(payload[29..33].try_into().unwrap());
    let total_chunks = u32::from_le_bytes(payload[33..37].try_into().unwrap());
    Ok(KatManifest {
        file_id,
        total_size,
        chunk_size,
        total_chunks,
    })
}

pub fn encode_chunk_payload(file_id: &FileId, idx: u32, total: u32, data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 1 + 16 + 4 + 4 + 4 + data.len());
    out.extend_from_slice(KAT_MAGIC);
    out.push(KAT_TYPE_CHUNK);
    out.extend_from_slice(file_id);
    out.extend_from_slice(&idx.to_le_bytes());
    out.extend_from_slice(&total.to_le_bytes());
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    out
}

pub fn try_decode_chunk_header(payload: &[u8]) -> Option<(FileId, u32, u32, usize)> {
    if payload.len() < 33 {
        return None;
    }
    if payload[0..4] != *KAT_MAGIC || payload[4] != KAT_TYPE_CHUNK {
        return None;
    }
    let mut file_id: FileId = [0u8; 16];
    file_id.copy_from_slice(&payload[5..21]);
    let idx = u32::from_le_bytes(payload[21..25].try_into().ok()?);
    let total = u32::from_le_bytes(payload[25..29].try_into().ok()?);
    Some((file_id, idx, total, 33))
}
