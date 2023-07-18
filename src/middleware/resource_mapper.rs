use crate::{ctx::Ctx, log::log_request, models::error::Error};
use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
                      }
            });
            println!("  ->> client_error_body: {client_error_body}");
            // Build the new response from the client_error_body
            // deref (*) here to have ownership (has Copy Trait)
            (*status_code, Json(client_error_body)).into_response()
        });
    // Build and log the server log line
    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
    println!("    ->> server log line - {uuid} - Error: {service_error:?}");
    println!();
    error_response.unwrap_or(res)
}
