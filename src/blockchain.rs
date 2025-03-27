use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::block::Block;
use crate::randomized_election::is_elected;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub nodes: Vec<String>,
    pub stored: HashMap<String, String>,
    pub balance: HashMap<String, f64>,
}

impl Blockchain {
    pub fn search_transaction(&self, id: &str) -> bool {
        let mut found: bool = false;
        for block in &self.chain {
            if let Some(tx) = &block.tx {
                match tx {
                    Transaction::FileStored(file_stored_tx) => {
                        found |= file_stored_tx.request_id == id;
                    }
                    _ => (),
                };
            }
        }

        return found;
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn verify_and_add(&self, blk: &Block) -> bool {
        let hash = match &blk.hash {
            Some(h) => h,
            None => {
                return false;
            }
        };

        if self.chain.last().unwrap().hash.unwrap() != hash.clone() {
            return false;
        }

        if !is_elected(blk.tx, hash, self.chain.len()) {}
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

    pub fn update(&mut self, new_chain: Blockchain) {
        // self.public_key_map.extend(new_chain.public_key_map);
        // TODO: implement this
        self.nodes.extend(new_chain.nodes);
        self.nodes.sort();
        self.nodes.dedup();
        if self.chain.len() < new_chain.chain.len() {
            self.chain = new_chain.chain;
        }
    }
}
