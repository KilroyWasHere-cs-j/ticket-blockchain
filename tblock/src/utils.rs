use crossterm::{
    ExecutableCommand,
    terminal::{self, ClearType},
    style::{self, Color, Stylize},
    cursor,
};
use std::io::{self, Write};

pub fn info(message: &str) {
    println!("INFO::::{:?}", message);
} 

pub fn sucess() {
    println!("BIG WIN");
}
