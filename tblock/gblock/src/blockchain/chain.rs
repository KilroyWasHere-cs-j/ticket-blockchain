use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;
use crate::ticket::Ticket;
use std::collections::VecDeque;

const MAX_BLOCKS: usize = 20000;

#[derive(Debug)]
pub struct BlockChain {
    pub chain: VecDeque<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_chain = Ticket::new();
        let genesis_block = Block::new(0, genesis_chain, String::from("0"), 0);
        BlockChain {
            chain: VecDeque::with_capacity(MAX_BLOCKS) 
        }
    }

    pub fn add_block(&mut self, block: Block) -> i64 {
        let mut action = 0;

        if *&self.chain.len() >= MAX_BLOCKS {
            let _ = &self.chain.pop_front();
            action = 2;
        }
        let _ = &self.chain.push_back(block);
        action
    }

    pub fn validate(&self, hash: String) -> bool{
        true
    } 
}
