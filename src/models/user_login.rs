use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    user_name: String,
    password_hash: String,
}

impl UserLogin {
    pub fn get_user_name(&self) -> &str {
        &self.user_name[..]
    }
}
