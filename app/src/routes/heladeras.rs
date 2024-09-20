use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn}, AppState,
};

use super::{Direccion, ParamsRecomendacion};

#[derive(Default, Serialize, Deserialize)]
struct RecomendacionHeladera {
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
        stock_minimo,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    Ok(Json(ubicacion))
}
