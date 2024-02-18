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

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

use crate::{
    ctx::Ctx,
    models::error::{Error, Result},
};

/// Extractor of ctx via From method
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
