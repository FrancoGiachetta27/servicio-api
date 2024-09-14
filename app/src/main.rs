use axum::{routing::get, Router};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod routes;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let nest_routes = Router::new()
        .route("/personas_vulnerables", get(todo!()))
        .route("/heladeras", get(todo!()));

    // build our application with a route
    let app = Router::new().nest("/api", nest_routes);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("error al crear el TcpListener");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Error al crear el servidor: {}", e);
    }
}
