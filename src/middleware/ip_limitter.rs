use crate::{
    config::Config,
    models::error::{Error, Result},
};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::Request, middleware::Next, response::Response};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};

#[derive(Clone, Debug)]
struct IpLimitter {
    ip_prefix: String,
    ip_whitelist: Option<String>,
    ip_blacklist: Option<String>,
}

impl IpLimitter {
    pub fn new() -> Self {
        Self {
            ip_prefix: String::new(),
            ip_whitelist: None,
            ip_blacklist: None,
        }
    }
}

pub async fn ip_limitter<B>(
    InsecureClientIp(insecure_ip): InsecureClientIp,
    // SecureClientIp(secure_ip): SecureClientIp,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - ip_limitter", "MIDDELWARE");
    println!("  >>> {insecure_ip:?}");
    // println!("  >>> {secure_ip:?}");
    Ok(next.run(req).await)
}
