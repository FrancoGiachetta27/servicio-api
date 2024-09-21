use crate::AppState;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod heladeras;
pub mod personas_vulnerables;
pub mod utils;

#[derive(Default, Serialize, Deserialize)]
struct Direccion {
    provincia: String,
    calle: String,
    altura: i32,
    latitud: f64,
    longitud: f64,
}

#[derive(Deserialize)]
pub struct ParamsRecomendacion {
    calle: String,
    altura: i16,
    provincia: Option<String>,
    radio_max: f64,
    stock_minimo: Option<i16>,
}

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/personas_vulnerables",
            get(personas_vulnerables::get_recomendacion),
        )
        .route("/heladeras", get(heladeras::get_recomendacion))
}
