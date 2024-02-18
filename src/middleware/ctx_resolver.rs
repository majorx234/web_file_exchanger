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

use axum::{middleware::Next, response::Response};
use http::{header, Request};

use crate::{
    ctx::Ctx,
    middleware::jwt_auth::parse_token,
    models::error::{Error, Result},
};

pub async fn ctx_resolver<B>(mut req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - ctx_resolver", "MIDDELWARE");
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
