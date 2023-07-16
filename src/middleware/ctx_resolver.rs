use axum::{
    middleware::Next,
    response::{IntoResponse, Response},
};
use http::{header, Request};

use crate::{
    ctx::Ctx,
    middleware::jwt_auth::parse_token,
    models::error::{Error, Result},
};

pub async fn ctx_resolver<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
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
    let result_ctx = match token
        .ok_or(Error::AuthFailNoAuthToken)
        .and_then(parse_token)
    {
        Ok((user, exp)) => {
            println!("token valid user: {user} exp: {exp}");
            Ok(Ctx::new(user))
        }
        Err(error) => Err(error),
    };
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthToken)) {
        req.headers_mut().remove(header::AUTHORIZATION);
    }
    // pass ctx via request extension
    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}
