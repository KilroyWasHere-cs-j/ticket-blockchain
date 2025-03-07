use gblock::build_block;
use gblock::blockchain::block::Block;
use crossterm::{
    ExecutableCommand,
    terminal::{self, ClearType},
    style::{self, Color, Stylize},
    cursor,
};
use std::io::{self, Write};
use chrono::Utc;

pub fn info(message: &str) {
    println!("{:?}<>INFO::::{:?}",Utc::now(), message);
}

pub fn Q(message: &str) {
    println!("{:?}", message);
}

pub fn sucess() {
    println!("BIG WIN");
}

pub fn prompt_for_new_block() -> Block{ 
    let mut firstName = String::new(); 
    let mut lastName = String::new();
    let mut showName = String::new();
    let mut theaterName = String::new();
    let mut seatLetter = String::new();
    let mut seatNumber = String::new();

    Q("Enter firstName");
    io::stdin().read_line(&mut firstName).expect("Failed");
    Q("Enter lastName");
    io::stdin().read_line(&mut lastName).expect("Failed");
    Q("Enter showName");
    io::stdin().read_line(&mut showName).expect("Failed");
    Q("Enter theaterName");
    io::stdin().read_line(&mut theaterName).expect("Failed");
    Q("Enter seatLetter");
    io::stdin().read_line(&mut seatLetter).expect("Failed");
    Q("Enter seatNumber");
    io::stdin().read_line(&mut seatNumber).expect("Failed");

    println!("------------------------------------------");
    println!("Built block:");
    println!("First Name:-> {}", &firstName);
    println!("Last Name:-> {}", &lastName);
    println!("Show Name:-> {}", &showName);
    println!("Theater Name:-> {}", &theaterName);
    println!("Seat Letter:-> {}", &seatLetter);
    println!("seatNumber:-> {}", &seatNumber);
    println!("------------------------------------------");

    let timestamp = Utc::now;
    let bloc = build_block(firstName, lastName, showName, theaterName, 'a', 10, "none".to_string());

       bloc
}
