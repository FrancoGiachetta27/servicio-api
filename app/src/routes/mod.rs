use crate::AppState;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod heladeras;
pub mod personas_vulnerables;

#[derive(Default, Serialize, Deserialize)]
struct Direccion {
    provincia: String,
    calle: String,
    altura: i16,
    latitud: f32,
    longitud: f32,
}

#[derive(Deserialize)]
pub struct ParamsRecomendacion {
    calle: String,
    altura: i16,
    provincia: Option<String>,
    radio_max: Option<i32>,
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
