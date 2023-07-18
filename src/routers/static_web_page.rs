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
