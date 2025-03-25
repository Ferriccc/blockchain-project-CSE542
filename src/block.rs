use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub previous_hash: Option<String>,
    pub tx: Option<Transaction>,
    pub hash: Option<String>,
}

impl Block {
    pub fn calculate_hash(&mut self) {
        let prv_hash = match self.previous_hash.clone() {
            Some(ph) => ph,
            None => "0".to_string(),
        };

        let tx = match self.tx.clone() {
            Some(tx) => format!("{:?}", tx),
            None => "0".to_string(),
        };

        let data = format!("{}{}", prv_hash, tx);
        let hash = format!("{:x}", Sha256::digest(data.as_bytes()));

        self.hash = Some(hash);
    }
}
