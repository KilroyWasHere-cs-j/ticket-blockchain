use std::error::Error;
use std::result::Result;
use crate::blockchain::chain::BlockChain;
use crate::ticket::Ticket;
use crate::blockchain::block::Block;
use chrono::Utc;

pub mod networking;
pub mod blockchain;
pub mod ticket;

/// Helper to build blocks
pub fn build_block(fname: String, lname: String, sname: String, tname: String, rlet: char, snum: i64, acom: String) -> Block {
    let ticket = Ticket::generate_new(fname, lname, sname, tname, rlet, snum, acom);
    Block::new(1, ticket, "gen_has".to_string(), 45)
}

// Helper to populate the blockchain only used in testing
pub fn pop_chain(mut blockchain: BlockChain) {
    let timestamp = Utc::now();
    
    let bloc = build_block("Gabriel".to_string(), "Tower".to_string(), "Ides of March".to_string(), "The Theater Project".to_string(), 'a', 10, "None".to_string());
 
    let action = blockchain.add_block(bloc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
