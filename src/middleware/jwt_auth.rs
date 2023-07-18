use crate::{
    config::Config,
    ctx::Ctx,
    models::{
        error::{Error, Result},
        token::Claims,
    },
};
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - auth", "MIDDELWARE");
    ctx?;
    Ok(next.run(req).await)
}

/// Parse a token of format `base64(header).base64(payload).signature`
/// - header :{"type":"jwt","alg":"HS256"}
/// - payload: {"user":"<username","exp":"<exp-time>" }
/// Returns (user_id, expiration)
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
            println! {"Error_message: {err}"};
            return Err(Error::AuthFailTokenInvalid);
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
