use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageTx {
    pub miner_id: String,
    pub request_id: String,
    pub file_hash: String,
    pub file_size: u64,
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
    pub file_content: u8,
}
