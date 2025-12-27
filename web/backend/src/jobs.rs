use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    PendingPayment,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub expected_amount_kas: f64,
    pub payment_address: String,
    pub payer_address: Option<String>,
    pub status: JobStatus,
    pub txid: Option<String>,
    pub block_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendJob {
    pub id: Uuid,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub to_address: String,
    pub amount_kas: f64,
    pub status: SendJobStatus,
    pub total_chunks: Option<u32>,
    pub submitted_chunks: u32,
    pub txid: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct JobStore {
    inner: Arc<RwLock<HashMap<Uuid, Job>>>,
}

impl JobStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, id: Uuid, job: Job) {
        let mut guard = self.inner.write().await;
        guard.insert(id, job);
    }

    #[allow(dead_code)]
    pub async fn get(&self, id: Uuid) -> Option<Job> {
        let guard = self.inner.read().await;
        guard.get(&id).cloned()
    }
}

#[derive(Clone, Default)]
pub struct SendJobStore {
    inner: Arc<RwLock<HashMap<Uuid, SendJob>>>,
}

impl SendJobStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, id: Uuid, job: SendJob) {
        let mut guard = self.inner.write().await;
        guard.insert(id, job);
    }

    pub async fn get(&self, id: Uuid) -> Option<SendJob> {
        let guard = self.inner.read().await;
        guard.get(&id).cloned()
    }

    pub async fn update<F>(&self, id: Uuid, f: F)
    where
        F: FnOnce(&mut SendJob),
    {
        let mut guard = self.inner.write().await;
        if let Some(job) = guard.get_mut(&id) {
            f(job);
        }
    }
}
