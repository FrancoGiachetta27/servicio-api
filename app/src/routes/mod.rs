use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

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
}

pub fn api_routes() -> Router {
    Router::new().route(
        "/personas_vulnerables",
        get(personas_vulnerables::get_recomendacion),
    )
}
