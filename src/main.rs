use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_file_exchanger::{backend::Backend, config::Config, database_interface::DataBaseInterface};

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
        .route("/", get(|| async { "hello, world" }))
        .layer(Extension(dbi));

    axum::Server::bind(&addr)
        .serve(routes_test.into_make_service())
        .await
        .expect("failed to start server");
}
