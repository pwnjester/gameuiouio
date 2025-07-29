// i need this to setup some turn based loop in which attacks are pulled through effects
// and reoccurring effects are checked every turn (for both sides)
use crate::character::create::Character;
use crate::effects::{apply_reoccurring_effects, HasEffects};
use crate::npcs::enemy::Enemy;
use crate::dialog::chat;
use std::fs;

pub fn load_character() -> Character {
    let raw = fs::read_to_string("data/character.json").unwrap();
    serde_json::from_str(&raw).unwrap()
}

pub fn load_enemy(name: &str) -> Enemy {
    let path = format!("data/npcs/{}.json", name);
    let raw = fs::read_to_string(&path).unwrap();
    serde_json::from_str(&raw).unwrap()
}

pub fn start(enemy_name: &str) {
    let mut player = load_character();
    let mut npc = load_enemy(enemy_name);

    println!("You encounter {}!", npc.name);

    loop {
        println!("\n--- Turn Start ---");
        println!("You: {} HP", player.hp);
        println!("{}: {} HP", npc.name, npc.hp);

        apply_reoccurring_effects(&mut player);
        apply_reoccurring_effects(&mut npc);

        let choice = chat("What do you do?", &["Attack", "Run"]);
        match choice {
            1 => {
                let dmg = 10;
                println!("You deal {} damage!", dmg);
                npc.hp = npc.hp.saturating_sub(dmg);
            }
            2 => {
                println!("You run away.");
                break;
            }
            _ => {}
        }

        if npc.hp == 0 {
            println!("You defeated {}!", npc.name);
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{} strikes back!", npc.name);
        let dmg = 5;
        player.hp = player.hp.saturating_sub(dmg);
        println!("You take {} damage!", dmg);

        if player.hp == 0 {
            println!("You died.");
            break;
        }
    }

    // optionally re-save the character
    let json = serde_json::to_string_pretty(&player).unwrap();
    fs::write("data/character.json", json).unwrap();
}

