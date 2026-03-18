use borsh::{BorshDeserialize, BorshSerialize};
use kaspa_bip32::{DerivationPath, ExtendedPrivateKey, Mnemonic, PrivateKey};
use kaspa_wallet_core::encryption::{Encryptable, EncryptionKind};
use kaspa_wallet_keys::secret::Secret;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};
use zeroize::Zeroize;

use crate::error::{AudioTransferError, Result};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, Zeroize)]
pub enum WalletSecretMaterial {
    MnemonicPhrase { phrase: String, password: String },
    PrivateKeyHex { hex: String },
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WalletProfileRecord {
    pub username: String,
    pub payload: Encryptable<WalletSecretMaterial>,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct WalletVaultFile {
    pub version: u32,
    pub profiles: Vec<WalletProfileRecord>,
}

impl Default for WalletVaultFile {
    fn default() -> Self {
        Self {
            version: 1,
            profiles: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct UnlockedProfile {
    username: String,
    secret: WalletSecretMaterial,
}

static UNLOCKED: OnceLock<Mutex<Option<UnlockedProfile>>> = OnceLock::new();

fn unlocked_state() -> &'static Mutex<Option<UnlockedProfile>> {
    UNLOCKED.get_or_init(|| Mutex::new(None))
}

fn secret_from_password(password: &str) -> Secret {
    Secret::new(password.as_bytes().to_vec())
}

fn find_profile<'a>(vault: &'a WalletVaultFile, username: &str) -> Option<&'a WalletProfileRecord> {
    vault.profiles.iter().find(|p| p.username == username)
}

fn default_data_dir() -> Result<PathBuf> {
    let base = dirs::data_local_dir().ok_or_else(|| {
        AudioTransferError::InvalidInput("Could not determine local data directory".to_string())
    })?;
    Ok(base.join("KaspaDataTransfer").join("wallet"))
}

fn vault_file_path() -> Result<PathBuf> {
    Ok(default_data_dir()?.join("vault.borsh"))
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    std::fs::create_dir_all(parent).map_err(AudioTransferError::Io)
}

fn load_vault() -> Result<WalletVaultFile> {
    let path = vault_file_path()?;
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(WalletVaultFile::default()),
        Err(e) => return Err(AudioTransferError::Io(e)),
    };

    borsh::from_slice::<WalletVaultFile>(&bytes).map_err(|e| {
        AudioTransferError::Serialization(format!("Failed to parse wallet vault file: {e}"))
    })
}

fn save_vault(v: &WalletVaultFile) -> Result<()> {
    let path = vault_file_path()?;
    ensure_parent_dir(&path)?;
    let bytes = borsh::to_vec(v).map_err(|e| {
        AudioTransferError::Serialization(format!("Failed to serialize wallet vault file: {e}"))
    })?;
    std::fs::write(path, bytes).map_err(AudioTransferError::Io)
}

pub fn list_profiles() -> Result<Vec<String>> {
    let v = load_vault()?;
    Ok(v.profiles.into_iter().map(|p| p.username).collect())
}

pub fn create_profile_mnemonic(
    username: &str,
    password: &str,
    word_count: u32,
    mnemonic_password: Option<&str>,
) -> Result<String> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "username is empty".to_string(),
        ));
    }
    if password.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "password is empty".to_string(),
        ));
    }

    let mut vault = load_vault()?;
    if find_profile(&vault, username).is_some() {
        return Err(AudioTransferError::InvalidInput(
            "profile already exists".to_string(),
        ));
    }

    let wc = match word_count {
        12 => kaspa_bip32::WordCount::Words12,
        24 => kaspa_bip32::WordCount::Words24,
        _ => {
            return Err(AudioTransferError::InvalidInput(
                "word_count must be 12 or 24".to_string(),
            ))
        }
    };

    let mnemonic = Mnemonic::random(wc, kaspa_bip32::Language::English).map_err(|e| {
        AudioTransferError::InvalidInput(format!("Mnemonic generation failed: {e}"))
    })?;
    let phrase = mnemonic.phrase().to_string();

    let secret = WalletSecretMaterial::MnemonicPhrase {
        phrase: phrase.clone(),
        password: mnemonic_password.unwrap_or("").to_string(),
    };
    let enc = Encryptable::from(secret)
        .into_encrypted(
            &secret_from_password(password),
            EncryptionKind::XChaCha20Poly1305,
        )
        .map_err(|e| AudioTransferError::Serialization(format!("Encryption failed: {e}")))?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });
    save_vault(&vault)?;

    Ok(phrase)
}

pub fn import_profile_mnemonic(
    username: &str,
    password: &str,
    phrase: &str,
    mnemonic_password: Option<&str>,
) -> Result<()> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "username is empty".to_string(),
        ));
    }
    if password.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "password is empty".to_string(),
        ));
    }
    let phrase = phrase.trim();
    if phrase.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "mnemonic phrase is empty".to_string(),
        ));
    }

    Mnemonic::new(phrase, kaspa_bip32::Language::English)
        .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid mnemonic: {e}")))?;

    let mut vault = load_vault()?;
    if find_profile(&vault, username).is_some() {
        return Err(AudioTransferError::InvalidInput(
            "profile already exists".to_string(),
        ));
    }

    let secret = WalletSecretMaterial::MnemonicPhrase {
        phrase: phrase.to_string(),
        password: mnemonic_password.unwrap_or("").to_string(),
    };
    let enc = Encryptable::from(secret)
        .into_encrypted(
            &secret_from_password(password),
            EncryptionKind::XChaCha20Poly1305,
        )
        .map_err(|e| AudioTransferError::Serialization(format!("Encryption failed: {e}")))?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });
    save_vault(&vault)
}

pub fn import_profile_private_key(
    username: &str,
    password: &str,
    private_key_hex: &str,
) -> Result<()> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "username is empty".to_string(),
        ));
    }
    if password.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "password is empty".to_string(),
        ));
    }
    let key = private_key_hex.trim();
    let bytes = hex::decode(key)
        .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid private key hex: {e}")))?;
    if bytes.len() != 32 {
        return Err(AudioTransferError::InvalidInput(
            "private key must be 32 bytes (64 hex chars)".to_string(),
        ));
    }

    let mut vault = load_vault()?;
    if find_profile(&vault, username).is_some() {
        return Err(AudioTransferError::InvalidInput(
            "profile already exists".to_string(),
        ));
    }

    let secret = WalletSecretMaterial::PrivateKeyHex {
        hex: key.to_string(),
    };
    let enc = Encryptable::from(secret)
        .into_encrypted(
            &secret_from_password(password),
            EncryptionKind::XChaCha20Poly1305,
        )
        .map_err(|e| AudioTransferError::Serialization(format!("Encryption failed: {e}")))?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });
    save_vault(&vault)
}

pub fn unlock_profile(username: &str, password: &str) -> Result<()> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "username is empty".to_string(),
        ));
    }
    if password.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "password is empty".to_string(),
        ));
    }

    let vault = load_vault()?;
    let p = find_profile(&vault, username)
        .ok_or_else(|| AudioTransferError::InvalidInput("profile not found".to_string()))?;

    let decrypted = p
        .payload
        .decrypt(Some(&secret_from_password(password)))
        .map_err(|e| AudioTransferError::InvalidInput(format!("Unlock failed: {e}")))?;
    let secret = decrypted.unwrap();

    let mut guard = unlocked_state().lock().map_err(|e| {
        AudioTransferError::Serialization(format!("wallet lock state poisoned: {e}"))
    })?;
    if let Some(mut unlocked) = guard.take() {
        unlocked.secret.zeroize();
    }
    *guard = Some(UnlockedProfile {
        username: username.to_string(),
        secret,
    });

    Ok(())
}

pub fn lock_wallet() {
    if let Ok(mut guard) = unlocked_state().lock() {
        if let Some(mut unlocked) = guard.take() {
            unlocked.secret.zeroize();
        }
    }
}

pub fn unlocked_username() -> Option<String> {
    unlocked_state()
        .lock()
        .ok()
        .and_then(|g| g.as_ref().map(|u| u.username.clone()))
}

pub fn derive_private_key_hex_for_profile(
    username: &str,
    password: &str,
    derivation_path: &str,
) -> Result<String> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "username is empty".to_string(),
        ));
    }
    if password.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "password is empty".to_string(),
        ));
    }
    let derivation_path = derivation_path.trim();
    if derivation_path.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "derivation_path is empty".to_string(),
        ));
    }

    let vault = load_vault()?;
    let p = find_profile(&vault, username)
        .ok_or_else(|| AudioTransferError::InvalidInput("profile not found".to_string()))?;
    let decrypted = p
        .payload
        .decrypt(Some(&secret_from_password(password)))
        .map_err(|e| AudioTransferError::InvalidInput(format!("Unlock failed: {e}")))?;
    let secret = decrypted.unwrap();

    match secret {
        WalletSecretMaterial::PrivateKeyHex { hex } => Ok(hex),
        WalletSecretMaterial::MnemonicPhrase { phrase, password } => {
            let mnemonic = Mnemonic::new(&phrase, kaspa_bip32::Language::English)
                .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid mnemonic: {e}")))?;
            let seed = mnemonic.to_seed(password.as_str());
            let path = DerivationPath::from_str(derivation_path).map_err(|e| {
                AudioTransferError::InvalidInput(format!("Invalid derivation path: {e}"))
            })?;
            let xprv = ExtendedPrivateKey::<kaspa_bip32::SecretKey>::new(seed)
                .map_err(|e| {
                    AudioTransferError::InvalidInput(format!("Invalid mnemonic seed: {e}"))
                })?
                .derive_path(&path)
                .map_err(|e| {
                    AudioTransferError::InvalidInput(format!("Key derivation failed: {e}"))
                })?;
            Ok(hex::encode(xprv.private_key().to_bytes()))
        }
    }
}

pub fn derive_private_key_hex_for_network(network: &str, derivation_path: &str) -> Result<String> {
    let network = network.trim();
    if network != "mainnet" && network != "testnet" && network != "devnet" {
        return Err(AudioTransferError::InvalidInput(
            "network must be one of: mainnet, testnet, devnet".to_string(),
        ));
    }
    let derivation_path = derivation_path.trim();
    if derivation_path.is_empty() {
        return Err(AudioTransferError::InvalidInput(
            "derivation_path is empty".to_string(),
        ));
    }

    let guard = unlocked_state().lock().map_err(|e| {
        AudioTransferError::Serialization(format!("wallet lock state poisoned: {e}"))
    })?;
    let Some(unlocked) = guard.as_ref() else {
        return Err(AudioTransferError::InvalidInput(
            "wallet is locked".to_string(),
        ));
    };

    match &unlocked.secret {
        WalletSecretMaterial::PrivateKeyHex { hex } => Ok(hex.clone()),
        WalletSecretMaterial::MnemonicPhrase { phrase, password } => {
            let mnemonic = Mnemonic::new(phrase, kaspa_bip32::Language::English)
                .map_err(|e| AudioTransferError::InvalidInput(format!("Invalid mnemonic: {e}")))?;
            let seed = mnemonic.to_seed(password.as_str());
            let path = DerivationPath::from_str(derivation_path).map_err(|e| {
                AudioTransferError::InvalidInput(format!("Invalid derivation path: {e}"))
            })?;
            let xprv = ExtendedPrivateKey::<kaspa_bip32::SecretKey>::new(seed)
                .map_err(|e| {
                    AudioTransferError::InvalidInput(format!("Invalid mnemonic seed: {e}"))
                })?
                .derive_path(&path)
                .map_err(|e| {
                    AudioTransferError::InvalidInput(format!("Key derivation failed: {e}"))
                })?;
            Ok(hex::encode(xprv.private_key().to_bytes()))
        }
    }
}
