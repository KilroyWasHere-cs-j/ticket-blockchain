use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;


pub struct Peer {
    pub id: i64,
    pub addr: String,
    pub port: String,
}

impl Peer {
    pub fn new(id: i64, addr: String, port: String) -> Self {
        Peer {
            id,
            addr,
            port
        }
    }
}

struct Host {
    id: String,
    ip: i64,
}

impl Host {
    pub fn new(id: String, ip: i64) -> Self {
        Host {
            id,
            ip
        }
    }
}

pub async fn serve(me: Peer) {
    let addr = format!("{}:{}", &me.addr, &me.port);
    println!("Serving at {}", addr);
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let mut know_hosts = Arc::new(Mutex::new(Vec::<Host>::new()));
    let mut connected_hosts = Arc::new(Mutex::new(Vec::<Host>::new()));
    let mut hosts = Arc::new(Mutex::new(0));

    loop {
        let know_hosts_clone = Arc::clone(&know_hosts);
        let connected_hosts_clone = Arc::clone(&connected_hosts);
        let hosts_cloned = Arc::clone(&hosts);

        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("--");

                let server_handle = tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                // This is run on disconnection
                                println!("Whoop");
                                return
                            }
                            Ok(n) => {
                                let mut h_tmp = hosts_cloned.lock().await;
                                let connected_host = Host::new(addr.to_string(), (*h_tmp + 1));
                                let mut cc_tmp = connected_hosts_clone.lock().await; 
                                cc_tmp.push(connected_host);

                                let msg = String::from_utf8_lossy(&buffer[..n]);
                                let msg = handle_message(msg.to_string());

                                if let Err(e) = socket.write_all(msg.as_bytes()).await {
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

pub async fn client(me: Peer) {
    let addr = format!("{}:{}", &me.addr, &me.port);
    println!("Ready for friends at {}", addr);
}

// 1 -> ACK
// 0 -> NO
// 1024 -> SYN
fn handle_message(msg: String) -> String {
    println!("{}", msg);
    msg
}

