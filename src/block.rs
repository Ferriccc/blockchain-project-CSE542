use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;

use crate::mempool::MemPool;
use crate::transaction::{FileStoredTx, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub previous_hash: String,
    pub tx: Transaction,
    pub hash: String,
}

impl Block {}
