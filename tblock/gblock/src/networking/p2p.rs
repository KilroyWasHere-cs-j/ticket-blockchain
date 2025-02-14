use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use std::error::Error;


pub struct Peer {
    pub id: i64,
    pub addr: String,
    pub port: String,
}

struct Host {
    id: i64,
    ip: String,
}

struct Local {
    know_hosts: Vec<Host>,
    connected_hosts: Vec<Host>,
}

impl Host {
    fn new(id: i64, ip: String) -> Self {
        Host {
            id,
            ip
        }
    }
}

impl Local {
    fn new(know_hosts: Vec<Host>, connected_hosts: Vec<Host>) -> Self {
        Local {
            know_hosts: vec![],
            connected_hosts: vec![],
        }
    }
}

pub async fn serve(me: Peer) {
    let addr = format!("{}:{}", &me.addr, &me.port);
    println!("Opening listener at {}", addr);
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let mut know_hosts: Vec<Host> = Vec::new();
    let mut connected_hosts: Vec<Host> = Vec::new();

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("--");
                println!("Got connection {:?}, {:?}", socket, addr);

                let server_handle = tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                println!("Building connection profile...");
                                let profile = Host { ip: addr.to_string(), id: 0 };
                                return
                            }
                            Ok(n) => { 
                                let msg = String::from_utf8_lossy(&buffer[..n]);
                                println!("Got {} from {}", msg, addr);

                                if let Err(e) = socket.write_all(b"Message recived/n").await {
                                    println!("Failed saying {}", e);
                                }
                            }
                            Err(e) => {
                                println!("Failed saying {}", e);
                            }
                        }
                    }
                });
            }
            Err(e) => {
                println!("Failed saying {}", e);
            }
        }
    }
}
