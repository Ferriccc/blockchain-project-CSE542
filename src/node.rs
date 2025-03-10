use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeInfo {
    pub id: String,
    pub public_key: Vec<u8>,
}
