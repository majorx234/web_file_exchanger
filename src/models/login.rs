use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    user_name: String,
    password_hash: String,
}
