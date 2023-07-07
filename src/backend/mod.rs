pub struct Backend {
    name: String,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            name: String::from("test_server"),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
