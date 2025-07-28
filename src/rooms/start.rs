use std::io::{self, Write};
use std::fs;
use serde_json::json;

use crate::dialog::*;

pub fn main() {
    println!("Welcome to this world or whatever");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("You see a bunch of stuff and whatever");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("You also see this weird looking guy with a mexican poncho");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("He approaches you...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let convo = chat("Hi there, I'm Robert", &["is that a real poncho? I mean, is that a mexican poncho or is that a sears poncho?", "hi", "bye"]);
    match convo {
        1 => println!("It's from instagram, I plan on using it as a beach towel"),
        2 => println!("Hello there"),
        3 => println!("What?"),
        _ => unreachable!(),
    }
}
