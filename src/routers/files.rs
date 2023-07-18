use crate::{
    config::Config, ctx::Ctx, models::error::Result, models::folder_structure::FolderStructure,
    server_state::ServerState,
};
use axum::{
    extract::multipart::Multipart,
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::StreamExt;

use serde_json::{json, Value};
use std::fs;

pub fn get_route() -> Router<ServerState> {
    Router::new()
        .route("/upload", post(handler_upload))
        .route("/files", get(handler_files_list))
}

pub async fn handler_files_list(_ctx: Ctx) -> Result<Json<Value>> {
    println!("->> {:12} - handler_files_list", "HANDLER");
    let paths = fs::read_dir(Config::new().get_file_store_dir_path()).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(Json(json!({ "msg": "files will come later" })))
}

async fn handler_upload(ctx: Ctx, mut multipart: Multipart) -> Result<Json<Value>> {
    println!("->> {:12} - handler_upload", "HANDLER");
    while let Some(field) = multipart.next_field().await.unwrap() {
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
