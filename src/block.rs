use serde::{Deserialize, Serialize};

use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub previous_hash: String,
    pub tx: Transaction,
    pub hash: String,
}

impl Block {}
