use std::array::from_fn;

use crate::{
    middleware::jwt_auth::auth,
    models::error::{Error, Result},
};
use axum::{
    extract::{Extension, Query},
    middleware,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct Info {
    info: Option<String>,
}

pub fn get_route() -> Router {
    Router::new()
        .route("/info", get(handler_info))
        .route_layer(middleware::from_fn(auth))
}

pub async fn handler_info(Query(params): Query<Info>) -> Result<Json<Value>> {
    println!("->> {:12} - handler_info - {params:?}", "HANDLER");
    let my_info = params.info.as_deref().unwrap_or("None");

    Ok(Json(json!({ "msg": my_info })))
}
