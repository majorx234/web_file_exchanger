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
    let mut frontend_dir_path = PathBuf::new();
    frontend_dir_path.push(std::env::var("FRONTEND_DIR").expect("FRONTEND_DIR not set"));
    match frontend_dir_path.is_absolute() {
        true => Router::new().nest_service("/", get_service(ServeDir::new(frontend_dir_path))),
        false => Router::new().nest_service(
            "/",
            get_service(ServeDir::new(
                std::env::current_dir().unwrap().join(frontend_dir_path),
            )),
        ),
    }
}
