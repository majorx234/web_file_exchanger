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

use crate::{ctx::Ctx, models::error::Result, server_state::ServerState};
use axum::{
    extract::{DefaultBodyLimit, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct Info {
    info: Option<String>,
}

pub fn get_route() -> Router<ServerState> {
    Router::new()
        .route("/info", get(handler_info))
        .layer(DefaultBodyLimit::max(1024))
}

pub async fn handler_info(ctx: Ctx, Query(params): Query<Info>) -> Result<Json<Value>> {
    println!("->> {:12} - handler_info - {params:?}", "HANDLER");
    let my_info = params.info.as_deref().unwrap_or("None");

    Ok(Json(
        json!({ "msg": my_info, "who ask for info?": format!("user: {}",ctx.get_user_name()) }),
    ))
}
