use crate::{ctx::Ctx, models::error::Result, server_state::ServerState};
use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct Info {
    info: Option<String>,
}

pub fn get_route() -> Router<ServerState> {
    Router::new().route("/info", get(handler_info))
}

pub async fn handler_info(ctx: Ctx, Query(params): Query<Info>) -> Result<Json<Value>> {
    println!("->> {:12} - handler_info - {params:?}", "HANDLER");
    let my_info = params.info.as_deref().unwrap_or("None");

    Ok(Json(
        json!({ "msg": my_info, "who ask for info?": format!("user: {}",ctx.get_user_name()) }),
    ))
}
