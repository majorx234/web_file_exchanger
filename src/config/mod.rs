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

use chrono::Duration;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub host_ip: String,
    pub port: u32,
    pub database_url: String,
    pub frontend_dir_path: PathBuf,
    pub file_store_dir_path: PathBuf,
    pub jwt_secret: String,
    pub jwt_expire_time: Duration,
    pub rust_log: String,
    pub username: String,
    pub password_hash: String,
}

impl Config {
    /// Config Constructor, reads env variables and sets config
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").expect("PORT not set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
        let jwt_expire_time = std::env::var("JWT_EXPIRE_TIME").expect("JWT_EXPIRE_TIME not set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let frontend_dir = std::env::var("FRONTEND_DIR").expect("FRONTEND_DIR not set");

        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let password_hash = std::env::var("PASSWORD_HASH").expect("PASSWORD_HASH not set");
        let mut frontend_dir_path = PathBuf::new();
        frontend_dir_path.push(frontend_dir);
        match frontend_dir_path.is_absolute() {
            true => (),
            false => {
                frontend_dir_path = std::env::current_dir().unwrap().join(frontend_dir_path);
            }
        };
        let file_store_dir = std::env::var("FILE_STORE_DIR").expect("FILE_STORE_DIR not set");
        // todo set folder where executed
        let rust_log =
            std::env::var("RUST_LOG").unwrap_or_else(|_| "todo_axum=debug,tower_http=debug".into());
        Config {
            host_ip,
            port: port.parse::<u32>().unwrap(),
            database_url,
            frontend_dir_path,
            file_store_dir_path: file_store_dir.into(),
            jwt_secret,
            jwt_expire_time: Duration::seconds(jwt_expire_time.parse::<i64>().unwrap()),
            rust_log,
            username,
            password_hash,
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }

    pub fn get_frontend_dir_path(&self) -> &Path {
        self.frontend_dir_path.as_path()
    }
    pub fn get_file_store_dir_path(&self) -> &Path {
        self.file_store_dir_path.as_path()
    }

    pub fn get_rust_log(&self) -> &str {
        &self.rust_log
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
    }
}
