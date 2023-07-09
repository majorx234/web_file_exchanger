use crate::{
    config::Config,
    models::error::{Error, Result},
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
        Some(_) => {
            return Ok(next.run(req).await);
        }
        None => {
            return Err(Error::AuthFail);
        }
    }
}
