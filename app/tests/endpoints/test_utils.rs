use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use servicio_apiV2::routes;
use servicio_apiV2::state::AppState;
use std::env;

use crate::common::migrate;

pub async fn setup_app() -> (Router, DatabaseConnection) {
    dotenv::dotenv().expect("could not load .env file");

    let db = {
        let db_url = env::var("DATABASE_LOCAL").unwrap();
        match Database::connect(db_url).await {
            Ok(db) => db,
            Err(e) => panic!("No se pudo conectar a la base de datos con url: {}", e),
        }
    };

    migrate(&db).await;

    let state = AppState::new(db.clone()).await.unwrap();

    (
        Router::new()
            .nest("/api", routes::api_routes())
            .with_state(state),
        db,
    )
}
