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

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    ctx::Ctx,
    models::error::{ClientError, Error, Result},
};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // -- User and context attributes.
    user_name: Option<String>,

    // -- http request attributes.
    req_path: String,
    req_method: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    req_methode: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: time_stamp.to_string(),
        user_name: ctx.map(|c| c.get_user_name().to_string()),
        req_path: uri.to_string(),
        req_method: req_methode.to_string(),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    println!("    ->> log_request: \n{}", json!(log_line));

    // TODO: send to log service
    Ok(())
}
