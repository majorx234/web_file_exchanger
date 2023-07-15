use crate::{
    config::Config,
    ctx::Ctx,
    middleware::jwt_auth::auth,
    models::error::{Error, Result},
    models::folder_structure::FolderStructure,
};
use axum::{
    extract::{multipart::Multipart, Extension, Query},
    middleware,
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::StreamExt;

use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;

pub fn get_route() -> Router {
    Router::new()
        .route("/upload", post(handler_upload))
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

async fn handler_upload(ctx: Ctx, mut multipart: Multipart) -> Result<Json<Value>> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "user:{} upload file:Length of `{}` is {} bytes",
            ctx.get_user_name(),
            file_name,
            data.len()
        );
    }
    Ok(Json(json!({ "msg": "files upload niy" })))
}
