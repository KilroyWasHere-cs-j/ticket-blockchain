use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;
use crate::ticket::Ticket;

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
}
