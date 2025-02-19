use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;
use crate::ticket::Ticket;

#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_chain = vec![Ticket::new()];
        let genesis_block = Block::new(0, genesis_chain, String::from("0"), 0);
        BlockChain {
            chain: vec![genesis_block] 
        }
    }

    pub fn add(&mut self, block: Block) {
        let _ = &self.chain.push(block);
    }

    pub fn validate(&self, hash: String) -> bool{
        true
    } 
}
