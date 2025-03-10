use libp2p::{gossipsub, identity};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::MyBehaviour;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedData {
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub id: String,
}

impl SignedData {
    pub fn new(
        data: Vec<u8>,
        private_key: &identity::Keypair,
        id: String,
    ) -> Result<Self, Box<dyn Error>> {
        let signature = private_key.sign(&data)?;
        Ok(SignedData {
            data,
            signature,
            id,
        })
    }

    pub fn broadcast(
        &self,
        swarm: &mut libp2p::Swarm<MyBehaviour>,
        topic: &gossipsub::IdentTopic,
    ) -> Result<(), Box<dyn Error>> {
        let data = serde_json::to_vec(self)?;
        swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic.clone(), data)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn verify(&self, public_key: &identity::PublicKey) -> bool {
        public_key.verify(&self.data, &self.signature)
    }
}
