// in the character folder i wanna have a "master" file so to speak
// and what itll do is:
// 1. check if character.json contains data
// 2. tell the user if they want to use the current data or if they want to overwrite the data
//    if the user wants to use it, it just loads the json
//    if the user wants to overwrite, it calls create.rs
//
// ok, time to figure out the game loop...
// i can have one file per room/stage of the game, so you start by running one room and then
// when you finish everything in that room, it calls the next room
// the pros of this is that its super modular and not too difficult to add a room to... (i think)
// the cons are that its more difficult to have dialog choices really effect the game
// i can prolly find a workaround though...
// i think we can have each room return a different result depending on the dialog chosen, and that result
// can influence the next room chosen or details in the coming rooms
// again, this is going to get pretty complicated, so idk, if a better solution is discovered,
// i will update this wall of comments
// i think im gonna save important choices made in the character.json and then just parse that in every room file
// then if the room file detects a certain important choice, itll change to a diff path which will call a different room
//
// for the non player characters now, i need a system for each one and their dialog/combat
// i think i can make a separate thing for bosses, but for the average npc idk
// i need to decide a few things about the actual dialog storage and setup.
// ok, i have to setup some sort of system to keep track of how friendly the player is with the character
// (if the character matters ofc), so i'll probably have some pub value that an npc struct can hold,
// and depending on the value, eg. bad, good, or meh, they will use a different dialog tree.
// i also need to add smth that will change these live, and i need a wider variety of values and dialog options
// (for the npcs, not the player)
//
// i do a have a few ideas for npcs:
// 1. for lower level enemies that dont talk, implement a randomizer that changes up their stats a bit
//    that way i can have one file for different enemies that dont really talk and just change the names
//    and one liners and dialog randomly, just by a bit

#[macro_use]
mod character;
mod world;

use crate::character::create;
use crate::world::rooms::one;

fn main() {
    character::master::main();
    world::rooms::one::main();
}
