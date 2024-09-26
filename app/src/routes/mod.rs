use crate::state::AppState;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

pub mod heladeras;
pub mod personas_vulnerables;
pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordenadas {
    latitud: f64,
    longitud: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Direccion {
    provincia: String,
    calle: String,
    altura: i32,
    coordenadas: Coordenadas
}

#[derive(Deserialize)]
pub struct ParamsRecomendacion {
    calle: String,
    altura: i16,
    provincia: Option<String>,
    radio_max: f64,
    stock_minimo: Option<i16>,
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/personas_vulnerables",
            get(personas_vulnerables::get_recomendacion),
        )
        .route("/heladeras", get(heladeras::get_recomendacion))
}
