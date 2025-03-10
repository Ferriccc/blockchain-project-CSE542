use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileStoredTx {
    pub miner_id: String,
    pub owner_id: String,
    pub file_hash: String,
    pub file_size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofOfStorgeTx {
    pub miner_id: String,
    pub file_hash: String,
    pub proof_of_storage: String,
    pub coins_earned: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Transaction {
    FileStored(FileStoredTx),
    ProofOfStorage(ProofOfStorgeTx),
}
