use std::thread;
use chrono::Utc;
use gblock;
use gblock::build_block;
use gblock::blockchain::chain::BlockChain;
use gblock::networking::p2p::{serve, Peer};

#[tokio::main]
async fn main() {
    println!("Welcome to Thea ticketing system node");
    println!("Initalizing...");

    let me = Peer::new(0, "127.0.0.1".to_string(), "8080".to_string());
    let mut blockchain = BlockChain::new();
    let mut blocs = 0;
    let mut loops = 0; // Literally a var to count loops...

    // Spawn networking stuff here 

    loop {
        let timestamp = Utc::now();
        let bloc = build_block("Gabriel".to_string(), "Tower".to_string(), "Ides of March".to_string(), "The Theater Project".to_string(), 'a', 10, "None".to_string());

        blocs += 1; 
        let action = blockchain.add_block(bloc);

        loops += 1;
    }
}

//
//fn update_term() {
//    println!("--------------------");
//    println!("{}", timestamp);
//    println!("{:?}", blockchain);
//    println!("--------------------");
//}

