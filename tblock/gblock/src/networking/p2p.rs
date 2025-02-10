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

pub async fn start(me: Peer) {
    let addr = format!("{}:{}", &me.addr, &me.port);
    println!("Opening listener at {}", addr);
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let mut this = Local::new(vec![], vec![]);

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("--");
                println!("Got connection {:?}, {:?}", socket, addr);

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                println!("Building connection profile...");
                                let profile = Host { ip: addr.to_string(), id: 0 };
                                // Push into Local
                                
                                match handle_init_connection() {
                                    Ok() => {
                                        println!("Handled new connection");
                                    }
                                    Err(e) => {
                                        println!("Failed saying {}", e);
                                    }
                                }
                                return;
                            }
                            Ok(n) => {
                                match handle_connection_message() {
                                    Ok() => {
                                        println!("Handled message");
                                    }
                                    Err(e) => {
                                        println!("Failed saying {}", e);
                                    }
                                } 
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

fn handle_init_connection() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn handle_connection_message(msg: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

