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
    ctx::Ctx,
    models::{
        error::{Error, Result},
        token::Claims,
    },
};
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{self, decode, errors::ErrorKind, DecodingKey, Validation};

/// Checks for context and thus for extraction of jwt token and athorization
/// * `ctx` ctx containing username
/// * `request` http request
/// * `Next` next middleware in chain
/// * `returns` response dependend on next middlewarecheck
pub async fn auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - auth", "MIDDELWARE");
    ctx?;
    Ok(next.run(req).await)
}

/// Parse a token of format `base64(header).base64(payload).signature`
/// `header`  example: {"type":"jwt","alg":"HS256"}
/// `payload` example: {"user":"<username","exp":"<exp-time>" }
/// * `jwt_token` base64 encoded token
/// * `returns` (user_id, expiration)
pub fn parse_token(jwt_token: String) -> Result<(String, usize)> {
    let token_header = match jsonwebtoken::decode_header(&jwt_token) {
        Ok(token_header) => token_header,
        Err(_) => {
            return Err(Error::AuthFailTokenWrongFormat);
        }
    };

    let user_claims = match decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(Config::new().jwt_secret.as_bytes()),
        &Validation::new(token_header.alg),
    ) {
        Ok(claims) => claims.claims,
        Err(err) => {
            if *err.kind() == ErrorKind::ExpiredSignature {
                return Err(Error::AuthFailTokenExpired);
            } else {
                return Err(Error::AuthFailTokenInvalid);
            }
        }
    };
    // TODO Check if user exist in database
    /*
        let user = match get_user_by_ref(user_ref, app_state.db_pool) {
            Ok(user) => user,
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    */
    Ok((user_claims.user, user_claims.exp))
}
