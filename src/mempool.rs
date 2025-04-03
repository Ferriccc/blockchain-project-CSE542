use futures::pending;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::{collections::VecDeque, error::Error, fs, fs::File};
use uuid::Uuid;

use crate::transaction::StorageTx;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemPool {
    pub pending: VecDeque<MemPoolRequest>,
    pub max_size: usize,
}

impl MemPool {
    pub fn add(&mut self, req: &MemPoolRequest) {
        if self.pending.len() == self.max_size {
            self.pending.pop_front();
        }

        self.pending.push_back(req.clone());
    }

    pub fn get_first(&mut self) -> Result<MemPoolRequest, Box<dyn Error>> {
        if let Some(req) = self.pending.front() {
            Ok(req.clone())
        } else {
            Err("No pending requests".into())
        }
    }
}

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
    pub fn mine(&self, id: &str) -> Result<StorageTx, Box<dyn Error>> {
        // store the file_content locally
        let mut fp = File::create(self.request_id.to_string())?;
        fp.write_all(&self.file_content)?;

        Ok(StorageTx {
            miner_id: id.to_string(),
            request_id: self.request_id.clone(),
            file_hash: self.clone().file_hash,
            file_size: self.clone().file_size,
        })
    }

    pub fn new(node_id: String, file_path: &str, reward: f64) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read(file_path)?;
        let file_hash = compute_file_hash(&file_content);
        let file_size = file_content.len() as u64;

        Ok(MemPoolRequest {
            node_id,
            request_id: Uuid::new_v4().to_string(),
            file_content,
            file_hash,
            file_size,
            reward,
        })
    }
}
