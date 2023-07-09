use crate::config::Config;
use axum::routing::get_service;
use axum::{
    extract::{Extension, Query},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn frontend() -> Router {
    let config = Config::new();
    let frontend_dir_path = config.get_frontend_dir_path();
    Router::new().nest_service("/", get_service(ServeDir::new(frontend_dir_path)))
}
