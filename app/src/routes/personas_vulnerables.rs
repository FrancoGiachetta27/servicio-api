use axum::{
    extract::{Query, State},
    Json,
};
use entity::repositories::Repository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
    AppState,
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
    State(state): State<Arc<AppState>>,
    Query(params): Query<ParamsRecomendacion>,
) -> Result<Json<GeoRefIn>, AppError> {
    let ParamsRecomendacion {
        calle,
        altura,
        provincia,
        radio_max,
        stock_minimo: _,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    let personas = state.personas_vulnerables_repo

    Ok(Json(ubicacion))
}
