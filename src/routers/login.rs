use crate::{
    config::Config,
    models::{
        error::{Error, Result},
        token::Claims,
        user_login::UserLogin,
    },
};

use axum::routing::get_service;
use axum::{
    extract::{Extension, Query},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::{json, Value};

pub fn get_route() -> Router {
    Router::new().route("/login", post(handler_login))
}

pub async fn handler_login(Json(user_login): Json<UserLogin>) -> Result<Json<Value>> {
    println!("->> {:12} - handler_login - {user_login:?}", "HANDLER");
    // TODO: Implement a real db/auth logic with JWT response
    if user_login.get_user_name() == "Heinz"
        && user_login.get_password_hash()
            == "f4d3ad4f524a2c260f3220d954abb08b7953a9a3998fd46a8a221c2bb2acf3c6"
    {
        let claims = Claims {
            user: user_login.get_user_name().to_string(),
            exp: "2024-01-01".to_string(),
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Config::new().jwt_secret.as_bytes()),
        )
        .unwrap();
        Ok(Json(json!({
            "msg": format!("hello, {}", user_login.get_user_name()),
                        "token":token
        })))
    } else {
        Err(Error::LoginFail)
    }
}
