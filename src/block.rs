use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transaction::{MonetaryTx, StorageTx};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub previous_hash: Option<String>,
    pub mtx: Option<MonetaryTx>,
    pub stx: Option<StorageTx>,
    pub hash: String,
}

impl Block {
    pub fn calculate_hash(mut self) -> Self {
        let prv_hash = match &self.previous_hash {
            Some(x) => x,
            None => "",
        };

        let stx = match &self.stx {
            Some(x) => &format!("{:?}", x),
            None => "",
        };

        let mtx = match &self.mtx {
            Some(x) => &format!("{:?}", x),
            None => "",
        };

        let data = format!("{}{}{}", prv_hash, stx, mtx);
        let hash = format!("{:x}", Sha256::digest(data.as_bytes()));

        self.hash = hash;

        self
    }
}
