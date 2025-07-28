use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::spells::Spell;

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub strength: u32,
    pub intelligence: u32,
    pub speed: u32,
    pub hp: u32,
    pub inventory: Vec<String>,
    pub spells: Vec<Spell>,
    pub money: u32,
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_class_stats(class: &str) -> (u32, u32, u32) {
    match class.to_lowercase().as_str() {
        "fighter" => (10, 2, 4),
        "wizard" => (2, 10, 3),
        "ninja" => (4, 3, 10),
        "idiot" => (0, 0, 0),
        _ => (1, 1, 1),
    }
}

pub fn main() {
    let name = prompt("Enter your name: ");
    let class = prompt("Choose your class (fighter, wizard, ninja): ");
    let (strength, intelligence, speed) = get_class_stats(&class);
    let spells = Spell::starter_spells_for(&class);

    let chara = Character {
        name,
        class,
        strength,
        intelligence,
        speed,
        hp: 50,
        inventory: vec![],
        spells,
        money: 50,
    };

    let json = serde_json::to_string_pretty(&chara).unwrap();
    println!("{}", json);
    std::fs::write("data/character.json", json).unwrap();
}

