use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub hp: u32, // changed from i32 to u32
    pub atk: i32,
    pub def: i32,
    pub dialog: Vec<String>,
    pub effects: Vec<String>, // add this line
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            name: "grubgunk".into(),
            hp: 20,
            atk: 5,
            def: 2,
            dialog: vec!["grubgunk snarls...".into()],
            effects: vec![], // add this line
        }
    }

    pub fn talk(&self) {
        for line in &self.dialog {
            println!("{}", line);
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    }
}