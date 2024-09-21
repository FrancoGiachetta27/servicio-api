use axum::{
    extract::{Query, State},
    Json,
};
use entity::{direccion, persona_vulnerable, prelude::Ubicacion, ubicacion};
use migration::sea_orm::{EntityTrait, QueryFilter};
use sea_orm::ColumnTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    errors::AppError,
    services::georef::{self, GeoRefIn},
    AppState,
};

use super::{utils::distancia_haversine, Direccion, ParamsRecomendacion};

#[derive(Default, Serialize, Deserialize)]
struct RecomendacionPersonaVulnerable {
    nombre: String,
    apellido: String,
    direccion: Direccion,
    cantidad_recomendada: i16,
}

impl RecomendacionPersonaVulnerable {
    pub fn new(
        persona: persona_vulnerable::Model,
        ubicacion: ubicacion::Model,
        direccion: direccion::Model,
        cantidad_recomendada: i16,
    ) -> Self {
        let direccion = Direccion {
            provincia: direccion.provincia,
            calle: direccion.calle,
            altura: direccion.altura,
            latitud: ubicacion.latitud,
            longitud: ubicacion.longitud,
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
    State(state): State<Arc<AppState>>,
    Query(params): Query<ParamsRecomendacion>,
) -> Result<Json<RecomendacionPersonaVulnerable>, AppError> {
    let ParamsRecomendacion {
        calle,
        altura,
        provincia,
        radio_max,
        stock_minimo: _,
    } = params;

    let georef_request = georef::request_georef(calle, altura, provincia)?;

    let ubicacion: GeoRefIn = georef_request.into_json()?;

    let persona_ubicacion = state
        .personas_vulnerables_repo
        .find_related(None, Ubicacion)
        .await?;

    let recomendaciones: Vec<RecomendacionPersonaVulnerable> = vec![];

    for (p, u) in persona_ubicacion.iter() {
        let u = u.first().unwrap();

        if distancia_haversine(u.latitud, u.longitud, u.latitud, u.longitud) <= radio_max {
            let (_, hijos) = state
                .personas_vulnerables_repo
                .find_self_related(Some(persona_vulnerable::Column::Uuid.eq(p.uuid)))
                .await?
                .first()
                .unwrap();

            let (ubicacion, direccion_opt) = Ubicacion::find()
                .filter(ubicacion::Column::Uuid.eq(u.uuid))
                .find_also_related(direccion::Entity)
                .one(&state.db)
                .await?
                .unwrap();

            let direccion = direccion_opt.unwrap();

            recomendaciones.push(RecomendacionPersonaVulnerable::new(
                *p,
                *u,
                direccion,
                hijos.len() as i16,
            ));
        }
    }

    Ok(Json(recomendaciones))
}
