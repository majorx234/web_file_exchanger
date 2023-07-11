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
    pub file_store_dir: PathBuf,
    pub jwt_secret: String,
    pub jwt_expire_time: Duration,
}

impl Config {
    pub fn new() -> Config {
        let host_ip = std::env::var("HOST_IP").expect("HOST_IP not set");
        let port = std::env::var("PORT").expect("PORT not set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
        let jwt_expire_time = std::env::var("JWT_EXPIRE_TIME").expect("JWT_EXPIRE_TIME not set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let frontend_dir = std::env::var("FRONTEND_DIR").expect("FRONTEND_DIR not set");

        let mut frontend_dir_path = PathBuf::new();
        frontend_dir_path.push(frontend_dir);
        match frontend_dir_path.is_absolute() {
            true => (),
            false => {
                frontend_dir_path = std::env::current_dir().unwrap().join(frontend_dir_path);
            }
        };
        let file_store_dir = std::env::var("FILE_STORE_DIR").expect("FILE_STORE_DIR not set");
        Config {
            host_ip,
            port: port.parse::<u32>().unwrap(),
            database_url,
            frontend_dir_path,
            file_store_dir: file_store_dir.into(),
            jwt_secret: "test".to_string(),
            jwt_expire_time: Duration::seconds(600),
        }
    }
    pub fn get_host_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host_ip, self.port)[..]).unwrap()
    }

    pub fn get_frontend_dir_path(&self) -> &Path {
        self.frontend_dir_path.as_path()
    }
}
