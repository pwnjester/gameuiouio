/// CHARACTER
// in the character folder i wanna have a "master" file so to speak
// and what itll do is:
// 1. check if character.json contains data
// 2. tell the user if they want to use the current data or if they want to overwrite the data
//    if the user wants to use it, it just loads the json
//    if the user wants to overwrite, it calls create.rs
//
/// GAME STRUCTURE
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
/// COMBAT
// i will implement the dialog system soon, but first, we need to come up with a combat system idea
// obviously, engaging in combat will have a negative effect on your relationship with an npc
// i think we could do a few things:
// 1. we could have something setup where there are spells and items that do things and character.json will track what
//    spells the player has and what the player is carrying
//    we would then simply have to implement a system for the damage each item/spell can do and maybe add effects
// 2. we could also just have a system where you use your strongest skill to do attacks, and it corresponds to dmg
//    making this interesting would be difficult, but its much less complex to implement and much simpler to keep track of
//    i probably won't do this one unless i figure out a way to make it interesting
// lets, right now assume im going with number 1.
// the structure would be, theres a folder for combat in which there will be items and spells
// obviously there can be non-combat related items in there too, its just for simplicity
// what i need to decide is whether i have one struct/json of all the items (and a separate one for all the spells)
// or if i should have a file for each. orrrrr, maybe just one mod.rs or smth
// either way im going to need to implement an effects file that stores a bunch of effects that the items and spells can steal from
// in that file, i can be forced to implement turn skipping and damage over multiple rounds etc
// first though, create a combat loop, because right now i dont have that. i need a turn based combat system
// and i need it to be pretty fun and interesting, which is reall tough when it comes to turn based
//
/// NPCs
// for the non player characters now, i need a system for each one and their dialog/combat
// i think i can make a separate thing for bosses, but for the average npc idk
// i need to decide a few things about the actual dialog storage and setup.
// ok, i have to setup some sort of system to keep track of how friendly the player is with the character
// (if the character matters ofc), so i'll probably have some pub value that an npc struct can hold,
// and depending on the value, eg. bad, good, or meh, they will use a different dialog tree.
// i also need to add smth that will change these live, and i need a wider variety of values and dialog options
// (for the npcs, not the player)
// i do a have a few ideas for npcs:
// 1. for lower level enemies that dont talk, implement a randomizer that changes up their stats a bit
//    that way i can have one file for different enemies that dont really talk and just change the names
//    and one liners and dialog randomly, just by a bit
// 2. a trading system for items and spells with certain npcs, but i want the loot to be specific to different areas
//    that way a player may have to go back to older rooms just to be able to move forward
//    (more playtime for less dev time :P)

#[macro_use]
mod character;
mod rooms;
mod dialog;
mod items;
mod spells;
mod npcs;

use crate::dialog::*;
use crate::items::*;
use crate::spells::*;
use crate::character::create;
use crate::rooms::*;
use crate::npcs::*;

fn main() {
    character::master::main();
    rooms::start::main();
}
