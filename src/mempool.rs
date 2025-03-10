use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemPoolRequest {
    pub owner_id: String,
    pub file_content: Vec<u8>,
    pub file_hash: String,
    pub file_size: u64,
}

impl MemPoolRequest {
    pub fn new(owner_id: String, file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read(file_path)?;
        let file_hash = utils::compute_file_hash(&file_content);
        let file_size = file_content.len() as u64;
        Ok(MemPoolRequest {
            owner_id,
            file_content,
            file_hash,
            file_size,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemPool {
    pub pending_txs: Vec<MemPoolRequest>,
}
