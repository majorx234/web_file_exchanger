use crate::models::error::Error;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::status;
use serde_json::json;
use uuid::Uuid;

use crate::models::error::ClientError;

pub async fn response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

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
            (*status_code, Json(client_error_body)).into_response()
        });
    // -- Todo:: Build and log the server log line
    println!("    ->> server log line - {uuid} - Error: {service_error:?}");
    println!();
    error_response.unwrap_or(res)
}
