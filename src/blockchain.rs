use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;

use crate::block::Block;
use crate::mempool::MemPool;
use crate::transaction::{FileStoredTx, Transaction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub nodes: Vec<String>,
    pub storage_map: HashMap<String, String>,
    pub balance_map: HashMap<String, f64>,
}

impl Blockchain {
    pub fn add_block(&mut self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        let last_block = self
            .chain
            .last()
            .ok_or_else(|| "cannot find last block in chain")?;

        let block_data = format!("{:?}{:?}", &last_block.hash, &tx);
        let hash = format!("{:x}", Sha256::digest(block_data.as_bytes()));

        let new_block = Block {
            previous_hash: last_block.clone().hash,
            tx: Some(tx),
            hash,
        };
        self.chain.push(new_block);

        Ok(())
    }

    pub fn verify(&self) -> bool {
        return true;

        // TODO: um.. this is probably the toughest part, will see..

        if self.chain.len() == 0 {
            return false;
        }
        if self.chain[0].hash != "0" {
            return false;
        }

        let mut act_balance: HashMap<String, u64> = HashMap::new();
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

    pub fn mine_from_mempool(&mut self, id: &str, mempool: &MemPool) -> Result<(), Box<dyn Error>> {
        let req = mempool
            .pending_txs
            .first()
            .ok_or_else(|| "cannot mine, empty mempool")?;

        let tx = FileStoredTx {
            miner_id: id.to_string(),
            owner_id: req.clone().owner_id,
            file_hash: req.clone().file_hash,
            file_size: req.clone().file_size,
        };

        // TODO: store the file_content locally

        self.add_block(Transaction::FileStored(tx))?;
        Ok(())
    }
}
