use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{error::Error, fs};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemPoolRequest {
    pub node_id: String,
    pub request_id: String,
    pub file_content: Vec<u8>,
    pub file_hash: String,
    pub file_size: u64,
    pub reward: f64,
}

fn compute_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    format!("{:x}", hasher.finalize())
}

impl MemPoolRequest {
    pub fn new(node_id: String, file_path: &str, reward: f64) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read(file_path)?;
        let file_hash = compute_file_hash(&file_content);
        let file_size = file_content.len() as u64;
        let request_id = Uuid::new_v4();
        Ok(MemPoolRequest {
            node_id,
            file_content,
            file_hash,
            file_size,
            reward,
        })
    }
}
