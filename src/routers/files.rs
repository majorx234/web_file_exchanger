use crate::{
    config::Config,
    middleware::jwt_auth::auth,
    models::error::{Error, Result},
    models::folder_structure::FolderStructure,
};
use axum::{
    extract::{Extension, Query},
    middleware,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;

pub fn get_route() -> Router {
    Router::new()
        .route("/files", get(handler_files_list))
        .route_layer(middleware::from_fn(auth))
}

pub async fn handler_files_list() -> Result<Json<Value>> {
    let paths = fs::read_dir(Config::new().get_file_store_dir_path()).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(Json(json!({ "msg": "files will come later" })))
}
