#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    MysteryMeat,
    DaggerOfRegret,
    ShinyRock,
}

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            Item::MysteryMeat => "mystery meat",
            Item::DaggerOfRegret => "dagger of regret",
            Item::ShinyRock => "shiny rock",
        }
    }

    pub fn price(&self) -> u32 {
        match self {
            Item::MysteryMeat => 3,
            Item::DaggerOfRegret => 20,
            Item::ShinyRock => 999,
        }
    }
}

