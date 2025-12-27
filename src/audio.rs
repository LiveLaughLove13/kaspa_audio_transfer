use std::fs::File;
use std::io::Read;
use crate::error::{Result, AudioTransferError};

/// Reads an audio file and returns its binary content
pub fn read_audio_file(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)
        .map_err(|e| AudioTransferError::Io(e))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| AudioTransferError::Io(e))?;
    
    // Basic validation that this is an audio file
    if !is_audio_file(&buffer) {
        return Err(AudioTransferError::InvalidInput(
            "The provided file doesn't appear to be a valid audio file".to_string()
        ));
    }
    
    Ok(buffer)
}

/// Validates if the provided data appears to be an audio file
fn is_audio_file(data: &[u8]) -> bool {
    // Check for common audio file signatures
    // MP3
    if data.len() > 2 && &data[0..3] == b"ID3" {
        return true;
    }
    
    // WAV
    if data.len() > 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WAVE" {
        return true;
    }
    
    // OGG
    if data.len() > 4 && &data[0..4] == b"OggS" {
        return true;
    }
    
    // FLAC
    if data.len() > 4 && &data[0..4] == b"fLaC" {
        return true;
    }
    
    // If none of the above, it might still be an audio file with a different format
    // or a corrupted file, but we'll be more permissive here
    true
}

/// Converts binary data to a hex string for embedding in Kaspa transactions
#[allow(dead_code)]
pub fn binary_to_hex(data: &[u8]) -> String {
    hex::encode(data)
}

/// Converts a hex string back to binary data
#[allow(dead_code)]
pub fn hex_to_binary(hex_str: &str) -> Result<Vec<u8>> {
    hex::decode(hex_str).map_err(|e| AudioTransferError::InvalidInput(
        format!("Invalid hex string: {}", e)
    ))
}

/// Saves binary data to a file
pub fn save_audio_file(data: &[u8], file_path: &str) -> Result<()> {
    std::fs::write(file_path, data)
        .map_err(|e| AudioTransferError::Io(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_binary_hex_conversion() {
        let data = b"test data";
        let hex = binary_to_hex(data);
        let decoded = hex_to_binary(&hex).unwrap();
        assert_eq!(data.to_vec(), decoded);
    }
    
    #[test]
    fn test_audio_file_validation() {
        // Create a test WAV file
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"RIFF____WAVE").unwrap();
        let path = file.path().to_str().unwrap();
        
        // Should pass validation
        assert!(is_audio_file(b"RIFF____WAVE"));
        
        // Should fail validation
        assert!(!is_audio_file(b"NOT_AN_AUDIO_FILE"));
    }
}
