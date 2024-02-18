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

use crate::models::error::{Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
use axum_client_ip::{InsecureClientIp, SecureClientIp};
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct IpLimitter {
    ip_prefix: String,
    ip_whitelist: Option<Vec<(String, i32)>>,
    ip_blacklist: Option<Vec<(String, i32)>>,
}

impl IpLimitter {
    pub fn new() -> Self {
        Self {
            ip_prefix: String::new(),
            ip_whitelist: None,
            ip_blacklist: None,
        }
    }

    pub fn create_iplimit_from_str(
        ip_whitelist_str: &str,
        _ip_blacklist_str: &str,
    ) -> Result<Self> {
        let mut ip_whitelist = Vec::new();
        let mut ip_blacklist = Vec::new();
        let ip_list_fct =
            |ip_list: &mut Vec<(String, i32)>, ip_list_str: &str, err: Error| -> Result<()> {
                for ip_range in ip_list_str.split(',') {
                    let mut ip_parts = ip_range.split('/');
                    let (count, _) = ip_parts.size_hint();
                    if count != 2 {
                        return Err(err);
                    }
                    let ip = ip_parts.next().unwrap();
                    let range = ip_parts.next().unwrap();
                    if let (Ok(_), Ok(range)) = (ip.parse::<Ipv4Addr>(), range.parse::<i32>()) {
                        ip_list.push((ip.to_string(), range));
                    }
                }
                Ok(())
            };
        ip_list_fct(
            &mut ip_whitelist,
            ip_whitelist_str,
            Error::ParseFailInvalidWhiteList,
        )?;
        ip_list_fct(
            &mut ip_blacklist,
            ip_whitelist_str,
            Error::ParseFailInvalidBlackList,
        )?;
        Ok(Self {
            ip_prefix: String::new(),
            ip_whitelist: Some(ip_whitelist),
            ip_blacklist: Some(ip_blacklist),
        })
    }
}

pub async fn ip_limitter<B>(
    InsecureClientIp(insecure_ip): InsecureClientIp,
    SecureClientIp(secure_ip): SecureClientIp,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - ip_limitter", "MIDDELWARE");
    println!("  >>> {insecure_ip:?}");
    println!("  >>> {secure_ip:?}");
    Ok(next.run(req).await)
}
