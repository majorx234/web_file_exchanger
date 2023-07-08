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
    models::error::{Error, Result},
    models::user_login::UserLogin,
    routers::{login, static_web_page},
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
        .merge(login::login_route())
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

#[derive(Debug, Deserialize)]
pub struct Info {
    info: Option<String>,
}

async fn handler_info(Query(params): Query<Info>) -> impl IntoResponse {
    println!("->> {:12} - handler_info - {params:?}", "HANDLER");
    let my_info = params.info.as_deref().unwrap_or("None");
    Html(format!("hello, {my_info}"))
}
