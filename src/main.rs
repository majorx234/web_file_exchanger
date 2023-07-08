use axum::{
    extract::{Extension, Query},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_file_exchanger::{
    backend::Backend, config::Config, database_interface::DataBaseInterface, models::login::Login,
    routers::static_web_page,
};

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut dbi = DataBaseInterface::new();
    dbi.add("Heinz".to_string(), "1234".to_string());

    let backend = Backend::new();
    println!("web_file_exchanger_server: {}", backend.get_name());
    println!(
        "is Heinz in db? {}",
        dbi.compare_password(&"Heinz".to_string(), &"1234".to_string())
    );
    let addr = config.get_host_socket_addr();
    println!("addr: {}", addr);

    let routes_test = Router::new()
        .route("/hello", get(handler_hello))
        .route("/info", get(handler_info))
        .route("/login", post(handler_login))
        .layer(Extension(dbi))
        .merge(static_web_page::frontend());
    axum::Server::bind(&addr)
        .serve(routes_test.into_make_service())
        .await
        .expect("failed to start server");
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");
    Html("hello, world")
}

#[derive(Debug, Deserialize)]
pub struct Info {
    info: Option<String>,
}

async fn handler_info(Query(params): Query<Info>) -> impl IntoResponse {
    println!("->> {:12} - handler_info - {params:?}", "HANDLER");
    let my_info = params.info.as_deref().unwrap_or("None");
    Html(format!("hello, {my_info}"))
}

async fn handler_login(Json(params): Json<Login>) -> impl IntoResponse {
    println!("->> {:12} - handler_login - {params:?}", "HANDLER");
    Html(format!("hello, {}", params.get_user_name()))
}
