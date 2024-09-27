use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::{ColumnTrait, Set};
use serde::{Deserialize, Serialize};

use entity::{
    direccion,
    heladera::{ActiveModel as ActiveHeladera, Column},
    repositories::Repository,
    ubicacion,
};
use uuid::Uuid;

use super::{
    utils::{distancia_haversine, guardar_ubicacion},
    AppState, Coordenadas, Ubicacion,
};
use crate::{
    errors::AppError,
    services::georef::{self, DireccionGeoRef},
};

use super::{Direccion, ParamsRecomendacion};

// Para poder deserializarlo con json despues
#[derive(Serialize, Deserialize)]
pub struct HeladeraInfo {
    direccion: Direccion,
    cantidad_viandas: u16,
}

impl HeladeraInfo {
    pub fn new(direccion: Direccion, cantidad_viandas: u16) -> Self {
        Self {
            direccion,
            cantidad_viandas,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HeladeraIn {
    pub heladeras: Vec<HeladeraInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HeladeraOut {
    pub ubicacion: Ubicacion,
    pub cantidad_viandas: i32,
}

impl HeladeraOut {
    pub fn new(cantidad_viandas: i32, ubicacion: Ubicacion) -> Self {
        Self {
            cantidad_viandas,
            ubicacion,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecomendacionHeladera {
    pub ubicacion: Ubicacion,
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
        };
        let ubicacion = Ubicacion {
            direccion,
            coordenadas,
        };

        Self {
            ubicacion,
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

    let ubicacion: DireccionGeoRef = georef_request.into_json()?;

    let ubicacion = ubicacion.direcciones.first().unwrap();

    let heladera_ubicacion = state
        .heladeras_repo
        .find_related(
            match stock_minimo {
                Some(sm) => Some(Column::CantidadViandas.gte(sm)),
                None => None,
            },
            ubicacion::Entity,
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
    Json(heladeras): Json<HeladeraIn>,
) -> Result<Json<Vec<HeladeraOut>>, AppError> {
    let ubicaciones = state.ubicaciones_repo.all().await?;

    let mut insersiones = Vec::new();

    for h in heladeras.heladeras {
        let direccion = h.direccion;

        let georef_response = georef::request_georef_direccion(
            direccion.calle,
            direccion.altura,
            Some(direccion.provincia),
        )?;
        let ubicacion_georef: DireccionGeoRef = georef_response.into_json()?;
        let ubicacion_georef = ubicacion_georef.direcciones.first().unwrap();

        let ubicacion_existente = ubicaciones
            .iter()
            .filter(|u| {
                u.latitud == ubicacion_georef.ubicacion.lat
                    && u.longitud == ubicacion_georef.ubicacion.lon
            })
            .last();

        let uuid_ubicacion = match ubicacion_existente {
            Some(u) => u,
            None => &guardar_ubicacion(&state, ubicacion_georef).await?,
        };
        let heladera_model = ActiveHeladera {
            uuid: Set(Uuid::new_v4().into()),
            direccion_id: Set(uuid_ubicacion.uuid.to_owned()),
            cantidad_viandas: Set(h.cantidad_viandas as i32),
        };

        let heladera_saved = state.clone().heladeras_repo.save(heladera_model).await?;

        insersiones.push(HeladeraOut::new(
            heladera_saved.cantidad_viandas,
            ubicacion_georef.into(),
        ));
    }

    Ok(Json(insersiones))
}
