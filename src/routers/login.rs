/*
 * This file is part of the web_file_exchanger distribution (https://github.com/majorx234/web_file_exchanger ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

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
use axum_extra::extract::WithRejection;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_route() -> Router<ServerState> {
    Router::new().route("/login", post(handler_login))
}

pub async fn handler_login(
    State(server_state): State<ServerState>,
    WithRejection(Json(user_login), _): WithRejection<Json<UserLogin>, Error>,
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
