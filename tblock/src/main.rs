use gblock;
use std::{thread, time};
use chrono::Utc;
use gblock::build_block;
use gblock::blockchain::chain::BlockChain;
use config::{Config, File};
use serde::Deserialize;
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
use lazy_static::lazy_static;
use indicatif::ProgressBar;
use std::time::Duration;
use crossterm::event::{self, Event};
use std::fs;
use std::io::prelude::*;
use std::sync::Arc;
use sysinfo::{Pid, Process, System};
use sysinfo::get_current_pid;
use std::sync::Mutex;
use std::borrow::Cow;

mod utils;
use crate::utils::{info, sucess};

lazy_static! {
    
}

#[derive(Debug, Deserialize)]
struct AppConfig<'a> {
    #[serde(borrow)]
    addr: Cow<'a, str>,
    port: usize,
    ascii_art_path: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    ui: u8,
    debug: u8,
    name: Option<String>,
    config_path: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut debug: u8 = 0;
    let mut ui: u8 = 0;

    let cli = Cli::parse();
    
    if let Some(name) = cli.name.as_deref() {
        info(name);
    }

    if let Some(config_path) = cli.config_path.as_deref() {
        info(config_path);
    }

    match cli.ui {
        0 => { ui = 0; info("UI will not be rendered"); },
        1 => { ui = 1; info("UI will be rendered"); },
        _ => { ui = 222; info("Yah that's not an option"); },
    }

    match cli.debug {
        0 => { debug = 0; info("Debug mode is off"); },
        1 => { debug = 1; info("Debug mode is kinda on"); },
        2 => { debug = 2; info("Debug mode is on"); },
        _ => { debug = 255; info("Bro what you try'n do"); },
    }

    info("Loading in config...");

    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Failed to load config");
    
    let config: AppConfig = config.try_deserialize().unwrap();

    info("Config loaded");
    
    thread::sleep(time::Duration::from_millis(1000));
   
    // Setup and config is done
    
    let blockchain = Arc::new(Mutex::new(BlockChain::new()));


    match fs::read_to_string(config.ascii_art_path) {
        Ok(art) => println!("{}", art),
        Err(e) => println!("{}", e),
    }

    info("Initalizing...");

    thread::sleep(time::Duration::from_millis(1000));

    info("Allocating data");

    info("Making blockchain");
    let blockchain_clone = Arc::clone(&blockchain);
    sucess();
    
    let chain_loop = thread::spawn(move || {
        loop {
            let timestamp = Utc::now();
    
            let bloc = build_block("gabriel".to_string(), "tower".to_string(), 
                "ides of march".to_string(), "the theater project".to_string(), 'a', 10, "none".to_string());

            blockchain_clone.lock().unwrap().add_block(bloc);
            let bLen = blockchain_clone.lock().unwrap().length;
            
       }
    });
    chain_loop.join().expect("failed to join chain_loop");
}
