use crate::{
    config::Config,
    models::error::{Error, Result},
};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::Request, middleware::Next, response::Response};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
pub struct IpLimitter {
    ip_prefix: String,
    ip_whitelist: Option<Vec<String>>,
    ip_blacklist: Option<Vec<String>>,
}

impl IpLimitter {
    pub fn new() -> Self {
        Self {
            ip_prefix: String::new(),
            ip_whitelist: None,
            ip_blacklist: None,
        }
    }

    pub fn create_iplimit_from_str(ip_whitelist_str: &str) -> Self {
        let mut ip_whitelist = Vec::new();
        for ip in ip_whitelist_str.split(',') {
            if ip.parse::<IpAddr>().unwrap().is_ipv4() {
                ip_whitelist.push(ip.to_string());
            }
        }
        Self {
            ip_prefix: String::new(),
            ip_whitelist: Some(ip_whitelist),
            ip_blacklist: None,
        }
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
