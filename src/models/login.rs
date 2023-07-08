use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Login {
    user_name: String,
    password_hash: String,
}

impl Login {
    pub fn get_user_name(&self) -> &str {
        &self.user_name[..]
    }
}
