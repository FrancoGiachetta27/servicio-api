use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::ColumnTrait;
use serde::{Deserialize, Serialize};

use entity::{direccion, heladera::Column, prelude::UbicacionEntity, ubicacion};

use super::{utils::distancia_haversine, AppState};
use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
};

use super::{Direccion, ParamsRecomendacion};

#[derive(Default, Serialize, Deserialize)]
pub struct RecomendacionHeladera {
    pub direccion: Direccion,
    pub cantidad_recomendada: u16,
}

impl RecomendacionHeladera {
    pub fn new(
        ubicacion: ubicacion::Model,
        direccion: direccion::Model,
        cantidad_recomendada: u16,
    ) -> Self {
        let direccion = Direccion {
            provincia: direccion.provincia,
            calle: direccion.calle,
            altura: direccion.altura,
            latitud: ubicacion.latitud,
            longitud: ubicacion.longitud,
        };

        Self {
            direccion,
            cantidad_recomendada,
        }
    }
}

pub async fn get_recomendacion(
    State(state): State<AppState>,
    Query(params): Query<ParamsRecomendacion>,
) -> Result<Json<Vec<RecomendacionHeladera>>, AppError> {
    let ParamsRecomendacion {
        calle,
        altura,
        provincia,
        radio_max,
        stock_minimo,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    let direccion_georef = ubicacion.direcciones;

    let heladera_ubicacion = state.heladeras_repo.find_related(Some(Column::CantidadViandas.gte(stock_minimo)), UbicacionEntity).await?;

    let mut recomendaciones: Vec<RecomendacionHeladera> = vec![];

    for (h, u) in heladera_ubicacion.into_iter() {
        let u = u.unwrap();

        if distancia_haversine(
            direccion_georef.ubicacion.lat,
            direccion_georef.ubicacion.lon,
            u.latitud,
            u.longitud,
        ) <= radio_max
        {
            let ubicacion_direccion = state
                .ubicaciones_repo
                .find_related(
                    Some(ubicacion::Column::Uuid.eq(u.uuid.clone())),
                    direccion::Entity,
                )
                .await?;

            let (_, direccion_opt) = ubicacion_direccion.first().unwrap();

            recomendaciones.push(RecomendacionHeladera::new(
                u,
                direccion_opt.clone().unwrap(),
                h.cantidad_viandas as u16,
            ));
        }
    }

    Ok(Json(recomendaciones))
}
