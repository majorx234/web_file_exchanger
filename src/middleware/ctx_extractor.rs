use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::{
    ctx::Ctx,
    models::error::{Error, Result},
};

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("--> {:12} - Ctx", "EXTACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthErrorCtxNotInRequestExt)?
            .clone()
    }
}
