use crate::models::user_login::UserLogin;

use axum::routing::get_service;
use axum::{
    extract::{Extension, Query},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};

pub fn login_route() -> Router {
    Router::new().route("/login", post(handler_login))
}

pub async fn handler_login(Json(params): Json<UserLogin>) -> impl IntoResponse {
    println!("->> {:12} - handler_login - {params:?}", "HANDLER");
    Html(format!("hello, {}", params.get_user_name()))
}
