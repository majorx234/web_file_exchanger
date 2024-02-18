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

use crate::config::Config;
use crate::server_state::ServerState;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

pub fn frontend() -> Router<ServerState> {
    let config = Config::new();
    let frontend_dir_path = config.get_frontend_dir_path();
    Router::new().nest_service("/", get_service(ServeDir::new(frontend_dir_path)))
}
