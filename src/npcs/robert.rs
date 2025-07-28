use crate::character::create::NPCRelationship;

pub struct Robert;

impl Robert {
    pub fn name() -> &'static str {
        "robert"
    }

    pub fn talk(friendliness: i32) {
        match friendliness {
            6..=i32::MAX => println!("'Hey, u seem cool. U on insta?'"),
            3..=5 => println!("'You don't even follow me do u?'"),
            1..=2 => println!("'Back up. You're ruining the vibe.'"),
            0 => println!("'THIS WAS A GIFT!' he throws the poncho and lunges at you."),
            _ => println!("he just silently blocks you"),
        }
    }
}

