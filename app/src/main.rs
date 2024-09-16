use axum::Router;
use errors::handle_404;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod errors;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("could not load .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    // build our application with a route
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .fallback(handle_404);
    // add documentation generator

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("error al crear el TcpListener");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Error al crear el servidor: {}", e);
    }
}
