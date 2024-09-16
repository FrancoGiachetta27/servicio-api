use axum::{routing::get, Router};

pub mod heladeras;
pub mod personas_vulnerables;

pub fn api_routes() -> Router {
    Router::new().route(
        "/personas_vulnerables",
        get(personas_vulnerables::get_recomendacion),
    )
}
