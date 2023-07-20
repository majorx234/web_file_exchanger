use crate::{
    config::Config,
    ctx::Ctx,
    models::folder_structure::FolderStructure,
    models::fs_cmd::Command,
    models::{error::Result, fs_cmd::FsCmd},
    server_state::ServerState,
};
use axum::{
    extract::multipart::Multipart,
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::StreamExt;

use serde_json::{json, Value};
use std::{fs, path::PathBuf, str::FromStr};

pub fn get_route() -> Router<ServerState> {
    Router::new()
        .route("/upload", post(handler_upload))
        .route("/files", get(handler_files_list).post(list_folder))
}

pub async fn handler_files_list(_ctx: Ctx) -> Result<Json<Value>> {
    println!("->> {:12} - handler_files_list", "HANDLER");
    let paths = fs::read_dir(Config::new().get_file_store_dir_path()).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(Json(json!({ "msg": "files will come later" })))
}

async fn list_folder(_ctx: Ctx, Json(_fs_cmd): Json<FsCmd>) -> Result<Json<Vec<FolderStructure>>> {
    let mut folder_structure: Vec<FolderStructure> = Vec::new();
    match _fs_cmd.cmd {
        Command::ls => {
            let relative_path = PathBuf::from_str(&_fs_cmd.path).unwrap();
            let mut full_path = PathBuf::new();
            full_path.push(Config::new().get_file_store_dir_path());
            full_path.push(relative_path.strip_prefix("/").unwrap());
            println!("file_store_path: {}", full_path.to_str().unwrap());
            let folder_items = fs::read_dir(full_path).unwrap();
            for item in folder_items {
                let folder_structure_item = FolderStructure {
                    filename: item
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    is_folder: item.unwrap().file_type().unwrap().is_dir(),
                    children: None,
                };
                folder_structure.push(folder_structure_item);
            }
        }
        Command::get => (),
    };
    Ok(Json(folder_structure))
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
