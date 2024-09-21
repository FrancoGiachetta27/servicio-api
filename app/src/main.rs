use axum::Router;
use entity::repositories::personas_vulnerables_repository::PersonaVulnerableRepository;
use errors::handle_404;
use migration::sea_orm::DatabaseConnection;
use std::env;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

mod errors;
mod routes;
mod services;
 use entity::repositories::heladeras_repository::HeladeraRepository;

#[derive(Clone)]
struct AppState {
    personas_vulnerables_repo: PersonaVulnerableRepository,
    heladeras_repo: HeladeraRepository,
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("could not load .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = env::var("PORT").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let state = Arc::new(init_state().await);

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

async fn init_state() -> AppState {
    let db = Database::connect(
        env::var("DATABASE_URL").expect("No se pudo conectarse a la base de datos"),
    )
    .await?;

    let personas_vulnerables_repo = PersonaVulnerableRepository::new(db).await.unwrap();
    let heladeras_repo = HeladeraRepository::new(db).await.unwrap();

    AppState {
        personas_vulnerables_repo,
        heladeras_repo,
        db,
    }
}
