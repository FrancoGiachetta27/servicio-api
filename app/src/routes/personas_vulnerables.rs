use axum::{
    extract::{Query, State},
    Json,
};
use entity::{
    direccion,
    persona_vulnerable::{self, ActiveModel as ActivePersona, SelfLinkHijos},
    repositories::Repository,
    ubicacion,
};
use sea_orm::{ColumnTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{utils::guardar_ubicacion, AppState, Coordenadas, Ubicacion};
use crate::{
    errors::AppError,
    services::georef::{self, DireccionGeoRef},
};

use super::{utils::distancia_haversine, Direccion, ParamsRecomendacion};

#[derive(Serialize, Deserialize)]
pub struct PersonaIn {
    pub nombre: String,
    pub apellido: String,
    pub direccion: Direccion,
    pub hijos: Option<Vec<PersonaIn>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PersonaOut {
    pub nombre: String,
    pub apellido: String,
    pub direccion: Ubicacion,
}

impl PersonaOut {
    pub fn new(nombre: String, apellido: String, direccion: Ubicacion) -> Self {
        Self {
            nombre,
            apellido,
            direccion,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecomendacionPersonaVulnerable {
    pub nombre: String,
    pub apellido: String,
    pub direccion: Ubicacion,
    pub cantidad_recomendada: u16,
}

impl RecomendacionPersonaVulnerable {
    pub fn new(
        persona: persona_vulnerable::Model,
        ubicacion: ubicacion::Model,
        direccion: direccion::Model,
        cantidad_recomendada: u16,
    ) -> Self {
        let direccion = Direccion {
            provincia: direccion.provincia,
            calle: direccion.calle,
            altura: direccion.altura,
        };
        let coordenadas = Coordenadas {
            latitud: ubicacion.latitud,
            longitud: ubicacion.longitud,
        };
        let direccion = Ubicacion {
            direccion,
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

    let georef_request = georef::request_georef_direccion(calle, altura, provincia)?;

    let ubicacion: DireccionGeoRef = georef_request.into_json()?;

    let direccion_georef = ubicacion.direcciones.first().unwrap();

    let persona_ubicacion = state
        .personas_vulnerables_repo
        .find_related(None, ubicacion::Entity)
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

pub async fn post_personas_vulnerables(
    State(state): State<AppState>,
    Json(personas): Json<Vec<PersonaIn>>,
) -> Result<Json<Vec<PersonaOut>>, AppError> {
    let ubicaciones = state.ubicaciones_repo.all().await?;

    let mut insersiones = Vec::new();

    for p in personas {
        let direccion = p.direccion;

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

        let uuid_padre = Uuid::new_v4();

        let persona_padre_model = ActivePersona {
            uuid: Set(Uuid::new_v4().into()),
            nombre: Set(p.nombre),
            apellido: Set(p.apellido),
            direccion_id: Set(uuid_ubicacion.uuid.to_owned()),
            pariente_a_cargo: Set(None),
        };

        let persona_saved = state
            .personas_vulnerables_repo
            .save(persona_padre_model)
            .await?;

        insersiones.push(PersonaOut::new(
            persona_saved.nombre,
            persona_saved.apellido,
            ubicacion_georef.into(),
        ));

        if let Some(hijos) = p.hijos {
            for h in hijos {
                let persona_hijo_model = ActivePersona {
                    uuid: Set(Uuid::new_v4().into()),
                    nombre: Set(h.nombre),
                    apellido: Set(h.apellido),
                    direccion_id: Set(uuid_ubicacion.uuid.clone()),
                    pariente_a_cargo: Set(Some(uuid_padre.clone().into())),
                };

                let hijo_saved = state
                    .personas_vulnerables_repo
                    .save(persona_hijo_model)
                    .await?;

                insersiones.push(PersonaOut::new(
                    hijo_saved.nombre,
                    hijo_saved.apellido,
                    ubicacion_georef.into(),
                ));
            }
        }
    }
    Ok(Json(insersiones))
}
