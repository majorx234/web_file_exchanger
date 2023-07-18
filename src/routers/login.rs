use crate::{
    config::Config,
    models::{
        error::{Error, Result},
        token::Claims,
        user_login::UserLogin,
    },
    server_state::ServerState,
};

use axum::{extract::State, routing::post, Json, Router};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_route() -> Router<ServerState> {
    Router::new().route("/login", post(handler_login))
}

pub async fn handler_login(
    State(server_state): State<ServerState>,
    Json(user_login): Json<UserLogin>,
) -> Result<Json<Value>> {
    println!("->> {:12} - handler_login - {user_login:?}", "HANDLER");
    // TODO: Implement a real db/auth logic with JWT response
    if server_state
        .dbi
        .compare_password(user_login.get_user_name(), user_login.get_password_hash())
    {
        let elapse_since_epoch =
            (SystemTime::now() + Duration::from_secs(300)).duration_since(UNIX_EPOCH);

        let claims = Claims {
            user: user_login.get_user_name().to_string(),
            exp: elapse_since_epoch.unwrap().as_secs() as usize,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Config::new().jwt_secret.as_bytes()),
        )
        .unwrap();

        // TODO: Use DTO-Model to pass response
        Ok(Json(json!({
            "msg": format!("hello, {}", user_login.get_user_name()),
            "token":token
        })))
    } else {
        Err(Error::LoginFail)
    }
}
