use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::{ColumnTrait, Set};
use serde::{Deserialize, Serialize};

use entity::{
    direccion::{self, ActiveModel as ActiveDireccion},
    heladera::{ActiveModel as ActiveHeladera, Column},
    prelude::UbicacionEntity,
    repositories::Repository,
    ubicacion::{self, ActiveModel as ActiveUbicacion},
};
use uuid::Uuid;

use super::{utils::distancia_haversine, AppState, Coordenadas};
use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
};

use super::{Direccion, ParamsRecomendacion};

#[derive(Serialize, Deserialize)]
struct HeladeraIn {
    nombre_ubicacion: String,
    coordenadas: Coordenadas,
    cantidad_viandas: u16,
}

#[derive(Serialize, Deserialize)]
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
        let coordenadas = Coordenadas {
            latitud: ubicacion.latitud,
            longitud: ubicacion.longitud,
        };

        let direccion = Direccion {
            provincia: direccion.provincia,
            calle: direccion.calle,
            altura: direccion.altura,
            coordenadas,
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

    let georef_request = georef::request_georef_direccion(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    let ubicacion = ubicacion.direcciones.first().unwrap();

    let heladera_ubicacion = state
        .heladeras_repo
        .find_related(
            Some(Column::CantidadViandas.gte(stock_minimo)),
            UbicacionEntity,
        )
        .await?;

    let mut recomendaciones: Vec<RecomendacionHeladera> = vec![];

    for (h, u) in heladera_ubicacion.into_iter() {
        let u = u.unwrap();

        if distancia_haversine(
            ubicacion.ubicacion.lat,
            ubicacion.ubicacion.lon,
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

pub async fn post_heladeras(
    State(state): State<AppState>,
    Json(heladeras): Json<Vec<HeladeraIn>>,
) -> Result<Json<u8>, AppError> {
    let heladeras: Vec<HeladeraIn> = heladeras;
    let ubicaciones = state.ubicaciones_repo.all().await?;

    for h in heladeras {
        let coordenaras = h.coordenadas;

        let georef_response =
            georef::request_georef_ubicacion(coordenaras.latitud, coordenaras.longitud)?;
        let ubicacion_georef: GeoRefIn = georef_response.into_json()?;
        let ubicacion_georef = ubicacion_georef.direcciones.first().unwrap();

        let ubicacion_existente = ubicaciones
            .iter()
            .filter(|u| {
                u.latitud == ubicacion_georef.ubicacion.lat
                    && u.longitud == ubicacion_georef.ubicacion.lon
            })
            .last();

        let uuid_ubicacion = match ubicacion_existente {
            Some(u) => u.uuid,
            None => {
                let direccion_model = ActiveDireccion {
                    uuid: Set(Uuid::new_v4().into()),
                    provincia: Set(ubicacion_georef.provincia.nombre),
                    calle: Set(ubicacion_georef.calle.nombre),
                    altura: Set(ubicacion_georef.altura.valor as i32),
                };

                let uuid_direccion = state.direccion_repo.save(direccion_model).await?.last_insert_id;
                
                let ubicacion_model = ActiveUbicacion {
                    uuid: Set(Uuid::new_v4().into()),
                    nombre: Set(h.nombre_ubicacion),
                    latitud: Set(h.coordenadas.latitud),
                    longitud: Set(h.coordenadas.longitud),
                    direccion_id: Set(uuid_direccion),
                };

                state
                    .ubicaciones_repo
                    .save(ubicacion_model)
                    .await?
                    .last_insert_id
            }
        };
        let heladera_model = ActiveHeladera {
            uuid: Set(Uuid::new_v4().into()),
            direccion_id: Set(uuid_ubicacion.clone().into()),
            cantidad_viandas: Set(h.cantidad_viandas as i16),
        };
    }

    Ok(Json(3u8))
}
