use std::io::{self, Write};
use std::fs;
use crate::character::create;

fn is_empty(path: &str) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len() == 0)
}

pub fn main() {
    if is_empty("data/character.json").unwrap_or(false) {
        create::main();
    } else {
        println!("You have a character saved!");
        print!("Do you want to continue with your character (1) or overwrite you character (2)? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string();

        if input == "1\n" {
            // load the character, idk what i'll do with this part yet
        } else if input == "2\n" {
            create::main();
        } else {
            println!("Invalid answer!");
            return;
        }
    }
}
