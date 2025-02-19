use std::error::Error;
use std::result::Result;
use crate::networking::p2p::{serve, client, Peer};
use crate::blockchain::chain::BlockChain;
use crate::ticket::Ticket;
use crate::blockchain::block::Block;

pub mod networking;
pub mod blockchain;
pub mod ticket;
pub mod codes;

/// Helper to build blocks
pub fn build_block(fname: String, lname: String, sname: String, tname: String, rlet: char, snum: i64, acom: String) -> Block {
    let mut tickets = Vec::new();

    tickets.push(Ticket::generate_new(fname, lname, sname, tname, rlet, snum, acom));

    Block::new(1, tickets, "gen_has".to_string(), 45)
}

// Helper to populate the blockchain only used in testing
fn pop_chain(blockchain: BlockChain) {

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
