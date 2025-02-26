use gblock;
use std::thread;
use chrono::Utc;
use gblock::build_block;
use gblock::blockchain::chain::BlockChain;
use gblock::networking::p2p::{serve, Peer};
use config::{Config, File};
use serde::Deserialize;
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
use lazy_static::lazy_static;
use indicatif::ProgressBar;
use std::time::Duration;
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};
use std::fs;
use std::io::prelude::*;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

lazy_static! {
    
}

struct SharedData { 
    chain_pop: i32,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    addr: String,
    port: usize,
    ascii_art_path: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    debug: u8,
    name: Option<String>,
    config_path: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if let Some(name) = cli.name.as_deref() {
        println!("{name}");
    }

    if let Some(config_path) = cli.config_path.as_deref() {
        println!("{config_path}");
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    println!("Loading in config...");

    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Failed to load config");
    
    let config: AppConfig = config.try_deserialize().unwrap();

    println!("Config loaded");



    // Setup and config is done
    
    let me = Peer::new(0, config.addr, config.port.to_string());
    let mut blockchain = BlockChain::new();
    let (tx, rx) = mpsc::channel(); 

    let contents = fs::read_to_string(config.ascii_art_path)
        .expect("Should have been able to read the file");

    println!("{}", contents);
    println!("Initalizing...");
    
    //let mut terminal = ratatui::init();
    
    let chain_loop = thread::spawn(move || {
        loop {
            let timestamp = Utc::now();
    
            let bloc = build_block("gabriel".to_string(), "tower".to_string(), "ides of march".to_string(), "the theater project".to_string(), 'a', 10, "none".to_string());
                tx.send(10).unwrap();
        }
    });
 
    let ui_loop = thread::spawn(move || {
        loop {
            println!("{}", rx.recv().unwrap());
        }
    });

    chain_loop.join().expect("Failed to join chain_loop");
    ui_loop.join().expect("Failed to join ui_loop");
}

