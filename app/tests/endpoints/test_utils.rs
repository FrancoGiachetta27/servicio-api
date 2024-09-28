use axum::Router;
use sea_orm::DatabaseConnection;
use servicio_apiV2::routes;
use servicio_apiV2::state::AppState;

pub async fn setup_app(db: DatabaseConnection) -> (Router, DatabaseConnection) {
    let state = AppState::new(db.clone()).await.unwrap();

    (
        Router::new()
            .nest("/api", routes::api_routes())
            .with_state(state),
        db,
    )
}
