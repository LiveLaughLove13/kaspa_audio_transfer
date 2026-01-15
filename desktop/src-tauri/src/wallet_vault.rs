use borsh::{BorshDeserialize, BorshSerialize};
use kaspa_wallet_core::encryption::{Encryptable, EncryptionKind};
use kaspa_wallet_keys::secret::Secret;
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, OnceLock},
};

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::network::NetworkType;
use kaspa_bip32::{DerivationPath, ExtendedPrivateKey, Mnemonic, PrivateKey};
use kaspa_utils::hex::FromHex;
use secp256k1::Keypair;
use zeroize::Zeroize;

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

#[derive(Clone, Debug, Serialize)]
pub struct WalletProfileInfo {
    pub username: String,
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

pub fn lock_wallet() {
    if let Ok(mut g) = unlocked_state().lock() {
        if let Some(mut u) = g.take() {
            u.secret.zeroize();
        }
    }
}

fn secret_from_password(password: &str) -> Secret {
    Secret::new(password.as_bytes().to_vec())
}

fn vault_dir(app_data_dir: &Path) -> Result<PathBuf, String> {
    let new_base = app_data_dir.join("KaspaDataTransfer").join("wallet");
    let old_base = app_data_dir.join("KaspaAudioTransfer").join("wallet");

    if new_base.exists() || !old_base.exists() {
        Ok(new_base)
    } else {
        Ok(old_base)
    }
}

fn vault_file_path(app_data_dir: &Path) -> Result<PathBuf, String> {
    let mut p = vault_dir(app_data_dir)?;
    p.push("vault.borsh");
    Ok(p)
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    let Some(parent) = path.parent() else { return Ok(()); };
    std::fs::create_dir_all(parent).map_err(|e| e.to_string())
}

fn load_vault(app_data_dir: &Path) -> Result<WalletVaultFile, String> {
    let path = vault_file_path(app_data_dir)?;
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(WalletVaultFile::default()),
        Err(e) => return Err(e.to_string()),
    };
    borsh::from_slice::<WalletVaultFile>(&bytes).map_err(|e| e.to_string())
}

fn save_vault(app_data_dir: &Path, v: &WalletVaultFile) -> Result<(), String> {
    let path = vault_file_path(app_data_dir)?;
    ensure_parent_dir(&path)?;
    let bytes = borsh::to_vec(v).map_err(|e| e.to_string())?;
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

pub fn list_profiles(app_data_dir: &Path) -> Result<Vec<WalletProfileInfo>, String> {
    let v = load_vault(app_data_dir)?;
    Ok(v
        .profiles
        .into_iter()
        .map(|p| WalletProfileInfo { username: p.username })
        .collect())
}

pub fn delete_profile(app_data_dir: &Path, username: &str) -> Result<(), String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("username is empty".to_string());
    }

    if get_unlocked_username().as_deref() == Some(username) {
        lock_wallet();
    }

    let mut vault = load_vault(app_data_dir)?;
    let before = vault.profiles.len();
    vault.profiles.retain(|p| p.username != username);
    if vault.profiles.len() == before {
        return Err("profile not found".to_string());
    }

    save_vault(app_data_dir, &vault)
}

pub fn clear_all_profiles(app_data_dir: &Path) -> Result<(), String> {
    lock_wallet();
    let path = vault_file_path(app_data_dir)?;
    match std::fs::remove_file(&path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn find_profile<'a>(vault: &'a WalletVaultFile, username: &str) -> Option<&'a WalletProfileRecord> {
    vault.profiles.iter().find(|p| p.username == username)
}

pub fn create_profile_mnemonic(app_data_dir: &Path, username: &str, password: &str, word_count: u32) -> Result<String, String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("username is empty".to_string());
    }
    let password = password;
    if password.is_empty() {
        return Err("password is empty".to_string());
    }

    let mut vault = load_vault(app_data_dir)?;
    if find_profile(&vault, username).is_some() {
        return Err("profile already exists".to_string());
    }

    // kaspa-bip32 supports only English wordlist and 12/24 words.
    let wc = match word_count {
        12 => kaspa_bip32::WordCount::Words12,
        24 => kaspa_bip32::WordCount::Words24,
        _ => return Err("word_count must be 12 or 24".to_string()),
    };

    let mnemonic = kaspa_bip32::Mnemonic::random(wc, kaspa_bip32::Language::English)
        .map_err(|e| e.to_string())?;

    let phrase = mnemonic.phrase().to_string();

    let secret = WalletSecretMaterial::MnemonicPhrase {
        phrase: phrase.clone(),
        password: "".to_string(),
    };

    let enc = Encryptable::from(secret)
        .into_encrypted(&secret_from_password(password), EncryptionKind::XChaCha20Poly1305)
        .map_err(|e| e.to_string())?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });

    save_vault(app_data_dir, &vault)?;
    Ok(phrase)
}

pub fn import_profile_mnemonic(
    app_data_dir: &Path,
    username: &str,
    password: &str,
    phrase: &str,
    mnemonic_password: Option<&str>,
) -> Result<(), String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("username is empty".to_string());
    }
    if password.is_empty() {
        return Err("password is empty".to_string());
    }
    let phrase = phrase.trim();
    if phrase.is_empty() {
        return Err("mnemonic phrase is empty".to_string());
    }

    // Validate.
    kaspa_bip32::Mnemonic::new(phrase, kaspa_bip32::Language::English).map_err(|e| e.to_string())?;

    let mut vault = load_vault(app_data_dir)?;
    if find_profile(&vault, username).is_some() {
        return Err("profile already exists".to_string());
    }

    let secret = WalletSecretMaterial::MnemonicPhrase {
        phrase: phrase.to_string(),
        password: mnemonic_password.unwrap_or("").to_string(),
    };

    let enc = Encryptable::from(secret)
        .into_encrypted(&secret_from_password(password), EncryptionKind::XChaCha20Poly1305)
        .map_err(|e| e.to_string())?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });

    save_vault(app_data_dir, &vault)
}

pub fn import_profile_private_key(app_data_dir: &Path, username: &str, password: &str, private_key_hex: &str) -> Result<(), String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("username is empty".to_string());
    }
    if password.is_empty() {
        return Err("password is empty".to_string());
    }

    let private_key_hex = private_key_hex.trim();
    let bytes = Vec::<u8>::from_hex(private_key_hex).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err("private key must be 32 bytes (64 hex chars)".to_string());
    }

    let mut vault = load_vault(app_data_dir)?;
    if find_profile(&vault, username).is_some() {
        return Err("profile already exists".to_string());
    }

    let secret = WalletSecretMaterial::PrivateKeyHex {
        hex: private_key_hex.to_string(),
    };

    let enc = Encryptable::from(secret)
        .into_encrypted(&secret_from_password(password), EncryptionKind::XChaCha20Poly1305)
        .map_err(|e| e.to_string())?;

    vault.profiles.push(WalletProfileRecord {
        username: username.to_string(),
        payload: enc,
    });

    save_vault(app_data_dir, &vault)
}

pub fn unlock_profile(app_data_dir: &Path, username: &str, password: &str) -> Result<(), String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("username is empty".to_string());
    }
    if password.is_empty() {
        return Err("password is empty".to_string());
    }

    let vault = load_vault(app_data_dir)?;
    let p = find_profile(&vault, username).ok_or_else(|| "profile not found".to_string())?;

    let decrypted = p
        .payload
        .decrypt(Some(&secret_from_password(password)))
        .map_err(|e| e.to_string())?;

    let secret = decrypted.unwrap();

    let mut g = unlocked_state().lock().map_err(|e| e.to_string())?;
    if let Some(mut u) = g.take() {
        u.secret.zeroize();
    }
    *g = Some(UnlockedProfile {
        username: username.to_string(),
        secret,
    });

    Ok(())
}

pub fn get_unlocked_username() -> Option<String> {
    unlocked_state().lock().ok().and_then(|g| g.as_ref().map(|u| u.username.clone()))
}

pub fn derive_keypair_for_path(network: &str, derivation_path: &str) -> Result<Keypair, String> {
    let network = network.trim();
    let derivation_path = derivation_path.trim();
    if derivation_path.is_empty() {
        return Err("derivation_path is empty".to_string());
    }

    let _nt = match network {
        "mainnet" => NetworkType::Mainnet,
        "testnet" => NetworkType::Testnet,
        "devnet" => NetworkType::Devnet,
        _ => return Err("network must be one of: mainnet, testnet, devnet".to_string()),
    };

    let guard = unlocked_state().lock().map_err(|e| e.to_string())?;
    let Some(unlocked) = guard.as_ref() else {
        return Err("wallet is locked".to_string());
    };

    match &unlocked.secret {
        WalletSecretMaterial::PrivateKeyHex { hex } => {
            let bytes = Vec::<u8>::from_hex(hex).map_err(|e| e.to_string())?;
            Keypair::from_seckey_slice(secp256k1::SECP256K1, &bytes)
                .map_err(|e| format!("invalid private key: {e}"))
        }
        WalletSecretMaterial::MnemonicPhrase { phrase, password } => {
            let mnemonic = Mnemonic::new(phrase, kaspa_bip32::Language::English).map_err(|e| e.to_string())?;
            let seed = mnemonic.to_seed(password.as_str());

            let path = DerivationPath::from_str(derivation_path).map_err(|e| e.to_string())?;
            let xprv = ExtendedPrivateKey::<kaspa_bip32::SecretKey>::new(seed)
                .map_err(|e| e.to_string())?
                .derive_path(&path)
                .map_err(|e| e.to_string())?;

            let bytes = xprv.private_key().to_bytes();
            Keypair::from_seckey_slice(secp256k1::SECP256K1, &bytes)
                .map_err(|e| format!("invalid derived private key: {e}"))
        }
    }
}

pub fn derive_receive_address(network: &str, derivation_path: &str) -> Result<String, String> {
    let network = network.trim();
    let derivation_path = derivation_path.trim();
    if derivation_path.is_empty() {
        return Err("derivation_path is empty".to_string());
    }

    let nt = match network {
        "mainnet" => NetworkType::Mainnet,
        "testnet" => NetworkType::Testnet,
        "devnet" => NetworkType::Devnet,
        _ => return Err("network must be one of: mainnet, testnet, devnet".to_string()),
    };

    let prefix: Prefix = nt.into();

    let keypair = derive_keypair_for_path(network, derivation_path)?;

    let xonly_pk = keypair.public_key().x_only_public_key().0;
    let addr = Address::new(prefix, Version::PubKey, &xonly_pk.serialize());
    Ok(addr.to_string())
}

pub fn debug_unlocked_material_fingerprint() -> Result<String, String> {
    let guard = unlocked_state().lock().map_err(|e| e.to_string())?;
    let Some(unlocked) = guard.as_ref() else {
        return Err("wallet is locked".to_string());
    };

    let mut m: HashMap<String, String> = HashMap::new();
    match &unlocked.secret {
        WalletSecretMaterial::PrivateKeyHex { hex } => {
            m.insert("type".to_string(), "private_key".to_string());
            m.insert("fingerprint".to_string(), hex.chars().take(8).collect());
        }
        WalletSecretMaterial::MnemonicPhrase { phrase, .. } => {
            m.insert("type".to_string(), "mnemonic".to_string());
            m.insert(
                "words".to_string(),
                phrase.split_whitespace().count().to_string(),
            );
        }
    }

    Ok(serde_json::to_string(&m).map_err(|e| e.to_string())?)
}
