use std::io::{self, Write};
use std::fs;
use serde_json::json;

use crate::dialog::*;
use crate::items::Item;
use crate::npcs::trader::Trader;
use crate::character::create::Character;

fn load_character() -> Character {
    let raw = std::fs::read_to_string("data/character.json").unwrap();
    serde_json::from_str(&raw).unwrap()
}

pub fn trader(mut player: &mut Character) {
    let mut shady = Trader::new("dirk");
    shady.add_item(Item::MysteryMeat);
    shady.add_item(Item::ShinyRock);

    shady.talk();
    shady.offer_trade(&mut player);
}

pub fn main() {
    let mut player = load_character();

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
        3 => trader(&mut player),
        _ => unreachable!(),
    }
}
