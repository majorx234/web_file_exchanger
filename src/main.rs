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

use std::sync::Arc;

use axum::{middleware, Router};
use axum_client_ip::SecureClientIpSource;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web_file_exchanger::{
    backend::Backend,
    config::Config,
    database::{test_db::TestDb, DataBaseInterface},
    file_indexer::FileIndex,
    middleware::{
        ctx_resolver::ctx_resolver, ip_limitter::ip_limitter, jwt_auth::auth,
        resource_mapper::response_mapper,
    },
    routers::{files, info, login, static_web_page},
    server_state::ServerElements,
};

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut dbi = TestDb::new();
    dbi.add(
        "Heinz".to_string(),
        "f4d3ad4f524a2c260f3220d954abb08b7953a9a3998fd46a8a221c2bb2acf3c6".to_string(),
    );
    let file_index = FileIndex::create_index(config.get_file_store_dir_path());

    let backend = Backend::new();
    let server_state = Arc::new(ServerElements::new(Box::new(dbi), Box::new(file_index)));
    let addr = config.get_host_socket_addr();
    println!("web_file_exchanger_server: {}", backend.get_name());
    println!("addr: {}", addr);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.get_rust_log()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_secure = Router::new()
        .merge(info::get_route())
        .merge(files::get_route())
        .route_layer(middleware::from_fn(auth))
        .route_layer(middleware::from_fn(ctx_resolver))
        .route_layer(middleware::from_fn(ip_limitter))
        .route_layer(SecureClientIpSource::ConnectInfo.into_extension());

    let routes_all = Router::new()
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
        .serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("failed to start server");
}
