use std::error::Error;
use std::result::Result;
use crate::networking::p2p::{start, Peer};

mod networking;
mod blockchain;
pub mod ticket;

pub fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn spinup() -> Result<(), Box<dyn Error>>{
    println!("Spinning up node...");
    println!("Switching to running...");
    run().await?;
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>>{
    let spindown = false;
    println!("Node running");

    let peer = Peer {
        id: 1,
        addr: "127.0.0.1".to_string(),
        port: "8080".to_string(),
    };
    start(peer).await;
    Ok(())
}

fn spindown() {
    todo!("Shuts the system down");
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
