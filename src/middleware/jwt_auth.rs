use crate::{
    config::Config,
    models::{
        error::{Error, Result},
        token::Claims,
    },
};
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    match token {
        Some(token) => {
            println!("found token: {}", token);
            match parse_token(token) {
                Ok((user, exp)) => {
                    println!("token valid user: {user} exp: {exp}");
                    return Ok(next.run(req).await);
                }
                Err(error) => return Err(error),
            };
        }
        None => {
            return Err(Error::AuthFailNoAuthToken);
        }
    }
}

/// Parse a token of format `base64(header).base64(payload).signature`
/// - header :{"type":"jwt","alg":"HS256"}
/// - payload: {"user":"<username","exp":"<exp-time>" }
/// Returns (user_id, expiration)
fn parse_token(jwt_token: String) -> Result<(String, String)> {
    let token_header = match jsonwebtoken::decode_header(&jwt_token) {
        Ok(token_header) => token_header,
        Err(_) => {
            return Err(Error::AuthFailTokenWrongFormat);
        }
    };

    let user_claims = match jsonwebtoken::decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(Config::new().jwt_secret.as_bytes()),
        &Validation::new(token_header.alg),
    ) {
        Ok(claims) => claims.claims,
        Err(_) => {
            return Err(Error::AuthFailTokenInvalid);
        }
    };
    // TODO Check exp time
    // TODO Check if user exist in database
    /*
        let user = match get_user_by_ref(user_ref, app_state.db_pool) {
            Ok(user) => user,
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    */
    Ok((user_claims.user, user_claims.exp))
}
