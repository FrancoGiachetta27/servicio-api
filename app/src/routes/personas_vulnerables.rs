use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
};

use super::{Direccion, ParamsRecomendacion};

#[derive(Default, Serialize, Deserialize)]
struct RecomendacionPersonaVulnerable {
    nombre: String,
    apellido: String,
    direccion: Direccion,
    cantidad_recomendada: i16,
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
