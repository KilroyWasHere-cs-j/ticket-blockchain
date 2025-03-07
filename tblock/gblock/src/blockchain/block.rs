use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::ticket::Ticket;
use sha256::digest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: usize,
    pub payload: Ticket,
    pub previous_hash: String,
    pub hash: String,
    pub nounce: usize,
}

impl Block {
    pub fn new(index: usize, payload: Ticket, previous_hash: String, nounce: usize) -> Self {
        let timestamp = Utc::now().timestamp() as usize;
        let hash = "Init".to_string();

        Block {
            index,
            timestamp,
            payload,
            previous_hash,
            hash,
            nounce
        }
    } 

    pub fn generate_hash(to_hash: String) -> String {
        "hashed".to_string()
    }

    pub fn self_hash(&self) -> String {
        digest(self.index.to_string())
    }

    pub fn f_print() {

    }
}
