#[derive(Debug)]
pub struct Friendly {
    pub name: String,
    pub dialog: Vec<String>,
    pub friendliness: i32,
}

impl Friendly {
    pub fn new() -> Self {
        Self {
            name: "elder mireleaf".into(),
            dialog: vec![
                "ah... a traveler...".into(),
                "tread softly in the glade.".into(),
            ],
            friendliness: 10,
        }
    }

    pub fn talk(&self) {
        for line in &self.dialog {
            println!("{}", line);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

