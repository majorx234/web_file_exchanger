use axum::{
    extract::{Extension, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_file_exchanger::{
    backend::Backend,
    config::Config,
    database_interface::DataBaseInterface,
    routers::{files, info, login, static_web_page},
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
        .merge(login::get_route())
        .merge(info::get_route())
        .merge(files::get_route())
        .merge(static_web_page::frontend())
        .layer(Extension(dbi))
        .layer(middleware::map_response(main_response_mapper));

    axum::Server::bind(&addr)
        .serve(routes_test.into_make_service())
        .await
        .expect("failed to start server");
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");
    Html("hello, world")
}
