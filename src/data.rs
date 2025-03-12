use libp2p::{gossipsub, identity};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::MyBehaviour;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub id: String,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub is_signed: bool,
}

impl Data {
    pub fn new(
        id: String,
        data: Vec<u8>,
        private_key: &identity::Keypair,
        public_key: Vec<u8>,
        is_signed: bool,
    ) -> Result<Self, Box<dyn Error>> {
        if !is_signed {
            return Ok(Data {
                id,
                data,
                signature: vec![],
                public_key,
                is_signed,
            });
        }
        let signature = private_key.sign(&data)?;
        Ok(Data {
            id,
            data,
            signature,
            public_key,
            is_signed,
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

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        let public_key = identity::PublicKey::try_decode_protobuf(&self.public_key)?;
        let expected_id = identity::PeerId::from_public_key(&public_key).to_string();
        if expected_id != self.id {
            return Ok(false);
        }
        Ok(public_key.verify(&self.data, &self.signature))
    }
}
