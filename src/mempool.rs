use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::{error::Error, fs, fs::File};
use uuid::Uuid;

use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::node::Node;
use crate::randomized_election::is_elected;
use crate::transaction::StorageTx;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemPoolRequest {
    pub node_id: String,
    pub request_id: String,
    pub file_content: Vec<u8>,
    pub file_hash: String,
    pub file_size: usize,
}

fn compute_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    format!("{:x}", hasher.finalize())
}

impl MemPoolRequest {
    pub fn mine(
        &self,
        node: &Node,
        blockchain: &mut Blockchain,
        total_nodes: usize,
    ) -> Result<Block, Box<dyn Error>> {
        if blockchain.search_transaction(&self.request_id) {
            return Err("Request already served".into());
        }

        if self.node_id == node.id {
            return Err("Requesting node is same as miner node".into());
        }

        let block = Block {
            previous_hash: Some(blockchain.chain.last().unwrap().hash.clone()),
            mtx: None,
            stx: Some(StorageTx {
                miner_id: node.id.clone(),
                request_id: self.request_id.clone(),
                file_hash: self.file_hash.clone(),
                file_size: self.file_size,
            }),
            hash: "".to_string(),
        }
        .calculate_hash();

        if !is_elected(&node.id, &block.hash.clone(), total_nodes as u64) {
            return Err("Not eligible to propose a block".into());
        }

        blockchain.add_block(block.clone());
        blockchain
            .stored
            .entry(self.request_id.clone())
            .or_insert(vec![])
            .push(node.id.clone());

        println!("{:#?}", blockchain.stored);

        // store the file_content locally
        let mut fp = File::create(self.request_id.to_string())?;
        fp.write_all(&self.file_content)?;

        println!("miner {} has mined request {}", self.request_id, node.id);

        Ok(block)
    }

    pub fn new(node_id: String, file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read(file_path)?;
        let file_hash = compute_file_hash(&file_content);
        let file_size = file_content.len();

        Ok(MemPoolRequest {
            node_id,
            request_id: Uuid::new_v4().to_string(),
            file_content,
            file_hash,
            file_size,
        })
    }
}
