use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::{header, request::Parts};

use crate::{
    ctx::Ctx, middleware::jwt_auth::parse_token, models::error::Error, models::error::Result,
};

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:12} - Ctx", "EXTACTOR");

        let token = parts
            .headers
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
                        return Ok(Ctx::new(user));
                    }
                    Err(error) => return Err(error),
                };
            }
            None => {
                return Err(Error::AuthFailNoAuthToken);
            }
        }
    }
}
