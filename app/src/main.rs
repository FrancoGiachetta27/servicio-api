use axum::Router;
use errors::handle_404;
use sea_orm::Database;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod errors;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("could not load .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let db = {
        let db_url = env::var("DATABASE_URL").unwrap();
        match Database::connect(db_url).await {
            Ok(db) => db,
            Err(e) => panic!("No se pudo conectar a la base de datos con url: {}", e),
        }
    };

    let state = match AppState::new(db).await {
        Ok(s) => s,
        Err(e) => panic!("No se pudo inicilizar el state: {}", e),
    };

    // build our application with a route
    let app = Router::new()
        .nest("/api", routes::api_routes())
        .with_state(state)
        .fallback(handle_404);
    // add documentation generator

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("error al crear el TcpListener");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Error al crear el servidor: {}", e);
    }
}
