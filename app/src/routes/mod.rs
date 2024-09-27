use crate::{services::georef, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

pub mod heladeras;
pub mod personas_vulnerables;
pub mod utils;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coordenadas {
    pub latitud: f64,
    pub longitud: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Direccion {
    pub provincia: String,
    pub calle: String,
    pub altura: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ubicacion {
    pub direccion: Direccion,
    pub coordenadas: Coordenadas,
}

impl From<&georef::Direccion> for Ubicacion {
    fn from(value: &georef::Direccion) -> Self {
        let direccion = Direccion {
            provincia: value.provincia.nombre.to_string(),
            calle: value.calle.nombre.to_string(),
            altura: value.altura.valor,
        };
        let coordenadas = Coordenadas {
            latitud: value.ubicacion.lat,
            longitud: value.ubicacion.lon,
        };
        Self {
            direccion,
            coordenadas,
        }
    }
}

#[derive(Deserialize)]
pub struct ParamsRecomendacion {
    calle: String,
    altura: i32,
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
        .route(
            "/personas_vulnerables",
            post(personas_vulnerables::post_personas_vulnerables),
        )
        .route("/heladeras", get(heladeras::get_recomendacion))
        .route("/heladeras", post(heladeras::post_heladeras))
}
