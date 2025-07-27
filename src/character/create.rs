use std::io::{self, Write};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Character {
    name: String,
    class: String,
    str_: u32,
    int_: u32,
    dex: u32,
    num: u32,
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_num(msg: &str) -> u32 {
    prompt(msg).parse().unwrap_or(0)
}

fn get_class_stats(class: &str) -> (u32, u32, u32) {
    match class.to_lowercase().as_str() {
        "fighter" => (10, 2, 4),
        "wizard" => (2, 10, 3),
        "ninja" => (4, 3, 10),
        "idiot" => (0, 0, 0),
        _ => (1, 1, 1), // default trash
    }
}

pub fn main() {
    let name = prompt("Enter your name: ");
    let class = prompt("Choose your class (fighter, wizard, ninja): ");
    let num = prompt_num("Enter your number: ");

    let (str_, int_, dex) = get_class_stats(&class);

    let chara = Character {
        name,
        class,
        str_,
        int_,
        dex,
        num,
    };

    let json = serde_json::to_string_pretty(&chara).unwrap();
    println!("{}", json);
    std::fs::write("data/character.json", json).unwrap();
}

