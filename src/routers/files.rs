/*
 * This file is part of the web_file_exchanger distribution (https://github.com/majorx234/web_file_exchanger ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{
    config::Config,
    ctx::Ctx,
    models::folder_structure::FolderStructure,
    models::fs_cmd::Command,
    models::{
        error::{Error, Result},
        fs_cmd::FsCmd,
    },
    server_state::ServerState,
};
use axum::{
    body::StreamBody,
    extract::{multipart::Multipart, DefaultBodyLimit, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::WithRejection;
use http::{header, HeaderMap};
// use path_absolutize::*;
use serde_json::{json, Value};
use std::io::Write;
use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
};
use std::{io::BufWriter, path::Component};
use tokio_util::io::ReaderStream;

pub fn get_route() -> Router<ServerState> {
    Router::new()
        .route("/upload", post(handler_upload))
        .route("/files/*file_path", get(handler_get_file))
        .route("/files", get(handler_list_files).post(cmd_on_folder))
}

pub async fn handler_get_file(_ctx: Ctx, Path(file_path): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_get_file: {}", "HANDLER", file_path);

    let relative_path = PathBuf::from_str(&file_path).unwrap();
    if relative_path
        .components()
        .any(|x| x == Component::ParentDir)
    {
        return Err(Error::InvalidAccessDirectoryTraversal);
    }

    let filename = match relative_path.file_name() {
        Some(name) => name,
        None => return Err(Error::InvalidFilePath),
    };

    let mut full_local_file_path = PathBuf::new();
    full_local_file_path.push(Config::new().get_file_store_dir_path());
    full_local_file_path.push(&relative_path);
    println!(
        ">>> full_local_file_path: {}",
        full_local_file_path.to_str().unwrap()
    );
    let file = match tokio::fs::File::open(&full_local_file_path).await {
        Ok(file) => file,
        Err(_err) => return Err(Error::FileNotFound),
    };

    // check against symlinks attacks
    let file_path_canonicalized = fs::canonicalize(&full_local_file_path);
    let file_store_canonicalized = fs::canonicalize(Config::new().get_file_store_dir_path());
    match file_path_canonicalized {
        Ok(file_path_canonicalized) => {
            if !file_path_canonicalized.starts_with(file_store_canonicalized.unwrap()) {
                return Err(Error::InvalidAccessEscapeBaseDir);
            }
        }
        Err(_) => return Err(Error::FileNotFound),
    }

    let content_type = match mime_guess::from_path(&full_local_file_path).first_raw() {
        Some(mime) => mime,
        None => return Err(Error::InvalidMimeType),
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{:?}\"", filename)
            .parse()
            .unwrap(),
    );

    let value = body;
    Ok(value)
}

pub async fn handler_list_files(_ctx: Ctx) -> Result<Json<Value>> {
    println!("->> {:12} - handler_list_files", "HANDLER");
    let paths = fs::read_dir(Config::new().get_file_store_dir_path()).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(Json(json!({ "msg": "files will come later" })))
}

async fn cmd_on_folder(
    _ctx: Ctx,
    State(server_state): State<ServerState>,
    WithRejection(Json(fs_cmd), _): WithRejection<Json<FsCmd>, Error>,
) -> Result<Json<Vec<FolderStructure>>> {
    let mut folder_structure: Vec<FolderStructure> = Vec::new();
    match fs_cmd.cmd {
        Command::ls => {
            let relative_path = PathBuf::from_str(&fs_cmd.path).unwrap();
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
        Command::find => {
            let relative_path = PathBuf::from_str(&fs_cmd.path).unwrap();
            let file_name = relative_path.file_name();
            // println!("search for filename: {:?}", file_name);
            // println!("file_index: {:?}", server_state.file_index);
            if let Some(file_name) = file_name {
                if let Some(file_name) = file_name.to_str() {
                    if let Some(result) = server_state.file_index.search(file_name) {
                        for item in result {
                            // println!("search_result: {:?}", result);
                            let folder_structure_item = FolderStructure {
                                filename: item,
                                is_folder: false,
                                children: None,
                            };
                            folder_structure.push(folder_structure_item);
                        }
                    }
                }
            }
        }
    };
    Ok(Json(folder_structure))
}

async fn handler_upload(
    ctx: Ctx,
    WithRejection(mut multipart, _): WithRejection<Multipart, Error>,
) -> Result<Json<Value>> {
    println!("->> {:12} - handler_upload", "HANDLER");
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            ">>> user:{} upload file:Length of `{}` is {} bytes",
            ctx.get_user_name(),
            file_name,
            data.len()
        );
        let relative_path = PathBuf::from_str(&file_name).unwrap();
        if relative_path
            .components()
            .any(|x| x == Component::ParentDir)
        {
            return Err(Error::InvalidAccessDirectoryTraversal);
        }
        let mut full_path = PathBuf::new();

        full_path.push(Config::new().get_file_store_dir_path());
        full_path.push(relative_path.strip_prefix("/").unwrap());
        //TODO path check
        //TODO: error handling
        if !full_path.starts_with(Config::new().get_file_store_dir_path()) {
            return Err(Error::InvalidAccessEscapeBaseDir);
        }
        println!(">>> file saved at {}", full_path.to_str().unwrap());
        let file = File::create(full_path).unwrap();
        let mut buf_writer = BufWriter::new(file);
        let written_bytes = buf_writer.write(&data).unwrap();
        println!(">>> wrote {} bytes", written_bytes);
        buf_writer.flush().unwrap();
    }
    Ok(Json(json!({ "msg": "files upload niy" })))
}
