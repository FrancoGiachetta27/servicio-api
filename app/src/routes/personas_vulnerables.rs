use axum::{
    extract::{Query, State},
    Json,
};
use entity::{
    direccion,
    persona_vulnerable::{self, SelfLinkHijos},
    prelude::UbicacionEntity,
    ubicacion,
};
use sea_orm::ColumnTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AppState, Coordenadas};
use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
};

use super::{utils::distancia_haversine, Direccion, ParamsRecomendacion};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecomendacionPersonaVulnerable {
    pub nombre: String,
    pub apellido: String,
    pub direccion: Direccion,
    pub cantidad_recomendada: u16,
}

impl RecomendacionPersonaVulnerable {
    pub fn new(
        persona: persona_vulnerable::Model,
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
            nombre: persona.nombre,
            apellido: persona.apellido,
            direccion,
            cantidad_recomendada,
        }
    }
}

pub async fn get_recomendacion(
    State(state): State<AppState>,
    Query(params): Query<ParamsRecomendacion>,
) -> Result<Json<Vec<RecomendacionPersonaVulnerable>>, AppError> {
    let ParamsRecomendacion {
        calle,
        altura,
        provincia,
        radio_max,
        stock_minimo: _,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    let direccion_georef = ubicacion.direcciones.first().unwrap();

    let persona_ubicacion = state
        .personas_vulnerables_repo
        .find_related(None, UbicacionEntity)
        .await?;

    let mut recomendaciones: Vec<RecomendacionPersonaVulnerable> = vec![];
    for (p, u) in persona_ubicacion.into_iter() {
        let u = u.unwrap();

        if distancia_haversine(
            u.latitud,
            u.longitud,
            direccion_georef.ubicacion.lat,
            direccion_georef.ubicacion.lon,
        ) <= radio_max
        {
            let persona_hijos = state
                .personas_vulnerables_repo
                .find_self_related(
                    Some(persona_vulnerable::Column::Uuid.eq(Uuid::from_slice(&p.uuid).unwrap())),
                    SelfLinkHijos,
                )
                .await?;

            let ubicacion_direccion = state
                .ubicaciones_repo
                .find_related(
                    Some(ubicacion::Column::Uuid.eq(u.uuid.clone())),
                    direccion::Entity,
                )
                .await?;

            let (_, hijos) = persona_hijos.first().unwrap();
            let (_, direccion_opt) = ubicacion_direccion.first().unwrap();

            let cantidad_viandas = hijos.len() + 1;

            recomendaciones.push(RecomendacionPersonaVulnerable::new(
                p,
                u,
                direccion_opt.clone().unwrap(),
                cantidad_viandas as u16,
            ));
        }
    }

    Ok(Json(recomendaciones))
}
