use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub stored: HashMap<String, Vec<String>>,
    pub balance: HashMap<String, f64>,
}

impl Blockchain {
    pub fn new_with_genesis_block() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            stored: HashMap::new(),
            balance: HashMap::new(),
        };

        blockchain.chain.push(
            Block {
                previous_hash: None,
                mtx: None,
                stx: None,
                hash: "".to_string(),
            }
            .calculate_hash(),
        );

        blockchain
    }

    pub fn search_transaction(&self, id: &str) -> bool {
        let mut found: bool = false;
        for block in &self.chain {
            if let Some(tx) = &block.stx {
                found |= tx.request_id == id;
            }
        }

        return found;
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn _verify_and_add(&self, _blk: &Block) -> bool {
        return true;
        // TODO: um.. this is probably the toughest part, will see..
        // Things to verify:
        // balances
        // signatures
        // random selection according own seed
        // exsisting mappings should not change
        // Extras:
        // instead of k = 1, use k > 1 and verify the PoST by majority votes
    }

    pub fn verify(&self) -> bool {
        // TODO: implement to verify a received blockchain is valid or not
        return true;
    }

    pub fn update(&mut self, new_chain: &mut Blockchain) {
        for (key, mut new_list) in new_chain.stored.drain() {
            self.stored
                .entry(key)
                .or_insert_with(Vec::new)
                .append(&mut new_list);
        }
        if self.chain.len() < new_chain.chain.len() {
            self.chain = new_chain.chain.clone();
        }
    }
}
