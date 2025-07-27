#[macro_use]
mod character;

use crate::character::create;

// in the character folder i wanna have a "master" file so to speak
// and what itll do is:
// 1. check if character.json contains data
// 2. tell the user if they want to use the current data or if they want to overwrite the data
//    if the user wants to use it, it just loads the json
//    if the user wants to overwrite, it calls create.rs

fn main() {
    character::master::main();
    // i want this to call the master file which will call create for me (i hope)
    // then load the json and call the rest of the game
}
