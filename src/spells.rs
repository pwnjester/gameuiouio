use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spell {
    Fireball,
    IceSpike,
    MindBlast,
}

impl Spell {
    pub fn name(&self) -> &'static str {
        match self {
            Spell::Fireball => "Fireball",
            Spell::IceSpike => "Ice Spike",
            Spell::MindBlast => "Mind Blast",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Spell::Fireball => "Deals hella fire dmg to one dude",
            Spell::IceSpike => "Slows target, sharp n cold",
            Spell::MindBlast => "Psychic burst, makes brains go wobbly",
        }
    }

    pub fn starter_spells_for(class: &str) -> Vec<Spell> {
        match class.to_lowercase().as_str() {
            "wizard" => vec![Spell::Fireball, Spell::MindBlast],
            _ => vec![],
        }
    }
}

