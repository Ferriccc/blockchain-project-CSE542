use libp2p::{gossipsub, identity};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{network::MyBehaviour, node::Node};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub node_id: String,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Data {
    pub fn broadcast<T: serde::Serialize>(
        node: &Node,
        data: T,
        swarm: &mut libp2p::Swarm<MyBehaviour>,
        topic: &gossipsub::IdentTopic,
    ) -> Result<(), Box<dyn Error>> {
        let data = serde_json::to_vec(&data)?;
        let signature = node.private_key.sign(&data)?;
        let data = Data {
            node_id: node.id.clone(),
            data,
            signature,
            public_key: node.public_key.clone(),
        };
        let data = serde_json::to_vec(&data)?;

        swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic.clone(), data)?;

        Ok(())
    }

    pub fn verify(&self) -> Result<bool, Box<dyn Error>> {
        let public_key = identity::PublicKey::try_decode_protobuf(&self.public_key)?;
        let expected_id = identity::PeerId::from_public_key(&public_key).to_string();
        if expected_id != self.node_id {
            return Ok(false);
        }
        Ok(public_key.verify(&self.data, &self.signature))
    }
}
