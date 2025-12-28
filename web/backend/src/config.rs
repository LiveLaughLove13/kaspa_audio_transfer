use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Config {
    pub kaspa_rpc_url: String,
    pub service_private_key: String,
    pub service_receive_address: String,
    pub backend_port: u16,
    pub upload_dir: PathBuf,
    pub kaspa_binary: String,
    pub max_upload_bytes: Option<usize>,
}

impl Config {
    pub fn from_env() -> Self {
        let kaspa_rpc_url = env::var("KASPA_RPC_URL").unwrap_or_else(|_| "grpc://127.0.0.1:16110".to_string());
        let service_private_key = env::var("SERVICE_PRIVATE_KEY").unwrap_or_else(|_| "".to_string());
        let service_receive_address = env::var("SERVICE_RECEIVE_ADDRESS").unwrap_or_else(|_| "".to_string());
        let backend_port = env::var("BACKEND_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);
        let upload_dir = env::var("UPLOAD_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("tmp/uploads"));

        // Path to the kaspa_audio_transfer binary. By default we rely on PATH,
        // but this can be overridden with an absolute path via env.
        let kaspa_binary = env::var("KASPA_AUDIO_TRANSFER_BIN")
            .ok()
            .and_then(|v| (!v.trim().is_empty()).then_some(v))
            .or_else(|| {
                let exe = if cfg!(windows) {
                    "kaspa_audio_transfer.exe"
                } else {
                    "kaspa_audio_transfer"
                };
                let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("..");
                let debug = repo_root.join("target").join("debug").join(exe);
                if fs::metadata(&debug).is_ok() {
                    return Some(debug.to_string_lossy().to_string());
                }
                let release = repo_root.join("target").join("release").join(exe);
                if fs::metadata(&release).is_ok() {
                    return Some(release.to_string_lossy().to_string());
                }
                None
            })
            .unwrap_or_else(|| "kaspa_audio_transfer".to_string());

        // Max allowed request body size for uploads (estimate/send/send_async).
        // - If MAX_UPLOAD_BYTES is unset or empty: disable the limit.
        // - If MAX_UPLOAD_BYTES is set to 0: disable the limit.
        // - If MAX_UPLOAD_BYTES is set to a positive integer: enforce that limit.
        let max_upload_bytes = match env::var("MAX_UPLOAD_BYTES") {
            Err(_) => None,
            Ok(s) => {
                let t = s.trim();
                if t.is_empty() {
                    None
                } else if let Ok(v) = t.parse::<u64>() {
                    if v == 0 {
                        None
                    } else {
                        Some(usize::try_from(v).unwrap_or(usize::MAX))
                    }
                } else {
                    None
                }
            }
        };

        Self {
            kaspa_rpc_url,
            service_private_key,
            service_receive_address,
            backend_port,
            upload_dir,
            kaspa_binary,
            max_upload_bytes,
        }
    }
}
