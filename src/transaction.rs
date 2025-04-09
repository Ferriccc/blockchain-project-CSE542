use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageTx {
    pub miner_id: String,
    pub request_id: String,
    pub file_hash: String,
    pub file_size: usize,
}

pub struct ProofOfStorageTx {
    pub asking_node_id: String,
    pub responding_node_id: String,
    pub start: usize,
    pub end: usize,
    pub proof_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonetaryTx {
    pub node_id: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryTx {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServeFileTx {
    pub request_id: String,
    pub file_content: Vec<u8>,
}
