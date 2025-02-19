use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::ticket::Ticket;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub payload: Vec<Ticket>,
    pub previous_hash: String,
    pub hash: String,
    pub nounce: u64,
}

impl Block {
    pub fn new(index: u64, payload: Vec<Ticket>, previous_hash: String, nounce: u64) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = "ah".to_string();

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

    pub fn f_print() {

    }
}
