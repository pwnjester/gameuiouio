// use crate::items::Item;
// use crate::npc::trader::Trader;
// use crate::character::create::Character;
//
// pub fn run_room(mut player: &mut Character) {
//     let mut shady = Trader::new("dirk");
//     shady.add_item(Item::MysteryMeat);
//     shady.add_item(Item::ShinyRock);
//
//     shady.talk();
//     shady.offer_trade(&mut player);
// }

use std::collections::HashMap;
use std::io::{self, Write};
use crate::items::Item;
use crate::character::create::Character;

#[derive(Debug)]
pub struct Trader {
    pub name: String,
    pub dialog: Vec<String>,
    pub inventory: HashMap<Item, u32>,
}

impl Trader {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            dialog: vec!["wanna trade or nah?".into()],
            inventory: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.insert(item.clone(), item.price());
    }

    pub fn show_goods(&self) {
        println!("{}'s shop:", self.name);
        for (i, (item, price)) in self.inventory.iter().enumerate() {
            println!("{}. {} - {}¢", i + 1, item.name(), price);
        }
    }

    pub fn talk(&self) {
        for line in &self.dialog {
            println!("{}", line);
            std::thread::sleep(std::time::Duration::from_millis(600));
        }
    }

    pub fn offer_trade(&self, player: &mut Character) {
        loop {
            self.show_goods();
            println!("You have: {}¢", player.money);
            print!("Pick an item number to buy (or 0 to exit): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim().parse::<usize>().unwrap_or(0);

            if choice == 0 {
                break;
            }

            if let Some((item, price)) = self.inventory.iter().nth(choice - 1) {
                if player.money >= *price {
                    player.money -= *price;
                    player.inventory.push(item.name().into());
                    println!("You bought a {}.", item.name());
                    let json = serde_json::to_string_pretty(&player).unwrap();
                    std::fs::write("data/character.json", json).unwrap();
                } else {
                    println!("You too broke for that.");
                }
            } else {
                println!("invalid choice.");
            }

            println!();
        }
    }
}

