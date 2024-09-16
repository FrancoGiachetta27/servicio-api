use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
};

#[derive(Default, Serialize, Deserialize)]
struct Direccion {
    provincia: String,
    calle: String,
    altura: i16,
    latitud: f32,
    longitud: f32,
}

#[derive(Default, Serialize, Deserialize)]
struct Recomendacion {
    nombre: String,
    apellido: String,
    direccion: Direccion,
    cantidad_recomendada: i16,
}

#[derive(Deserialize)]
pub struct ParamsRecomendacion {
    calle: String,
    altura: i16,
    provincia: Option<String>,
    radio_max: Option<i32>,
}

pub async fn get_recomendacion(
    Query(params): Query<ParamsRecomendacion>,
) -> Result<Json<GeoRefIn>, AppError> {
    let ParamsRecomendacion {
        calle,
        altura,
        provincia,
        radio_max,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    Ok(Json(ubicacion))
}
