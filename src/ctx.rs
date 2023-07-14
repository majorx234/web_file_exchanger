#[derive(Clone, Debug)]
pub struct Ctx {
    user_name: String,
}

impl Ctx {
    pub fn new(user_name: String) -> Self {
        Self { user_name }
    }
    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}
