use std::sync::Arc;

use axum::{
    extract::{Extension, Query, State},
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
    database::{test_db::TestDb, DataBaseInterface},
    middleware::{
        ctx_resolver::{self, ctx_resolver},
        jwt_auth::auth,
        resource_mapper::response_mapper,
    },
    routers::{files, info, login, static_web_page},
    server_state::{ServerElements, ServerState},
};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut dbi = TestDb::new();
    dbi.add(
        "Heinz".to_string(),
        "f4d3ad4f524a2c260f3220d954abb08b7953a9a3998fd46a8a221c2bb2acf3c6".to_string(),
    );

    let backend = Backend::new();
    println!("web_file_exchanger_server: {}", backend.get_name());
    println!(
        "is Heinz in db? {}",
        dbi.compare_password(&"Heinz".to_string(), &"1234".to_string())
    );
    let server_state = Arc::new(ServerElements::new(Box::new(dbi)));
    let addr = config.get_host_socket_addr();
    println!("addr: {}", addr);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.get_rust_log()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_secure = Router::new()
        .merge(info::get_route())
        .merge(files::get_route())
        .route_layer(middleware::from_fn(auth))
        .route_layer(middleware::from_fn(ctx_resolver));

    let routes_all = Router::new()
        .route("/hello", get(handler_hello))
        .merge(login::get_route())
        .merge(router_secure)
        .merge(static_web_page::frontend())
        .layer(middleware::map_response(response_mapper))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(server_state);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .expect("failed to start server");
}

async fn handler_hello(State(server_state): State<ServerState<'_>>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");
    Html("hello, world")
}
