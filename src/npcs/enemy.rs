#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub dialog: Vec<String>,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            name: "grubgunk".into(),
            hp: 20,
            atk: 5,
            def: 2,
            dialog: vec!["grubgunk snarls...".into()],
        }
    }

    pub fn talk(&self) {
        for line in &self.dialog {
            println!("{}", line);
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    }
}

