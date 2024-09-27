use std::vec;

use axum_test::TestServer;
use rstest::rstest;
use serde_json::json;
use serial_test::file_serial;
use servicio_apiV2::routes::{
    personas_vulnerables::{PersonaIn, PersonaInfo, PersonaOut, RecomendacionPersonaVulnerable},
    Coordenadas, Direccion, Ubicacion,
};
use tokio;

use crate::common::TestContext;

use super::test_utils::setup_app;

#[rstest]
#[case("Medrano", 500, Some("CABA"), 10.0, vec![
    ("Maria", "Perez", 2),
    ("Nicole", "Perez", 1),
    ("Florencia", "Perez", 1)
])]
#[tokio::test]
#[file_serial]
async fn test_personas_endpoints_recomendaciones(
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: Option<&str>,
    #[case] radio_max: f64,
    #[case] recomendaciones_esperadas: Vec<(&str, &str, u16)>,
) {
    let ctx = TestContext::setup_with_migration().await;

    let (app, _) = setup_app(ctx.db.clone()).await;

    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/api/personas_vulnerables")
        .add_query_params(match provincia {
            Some(p) => json!({
                "calle": calle,
                "altura": altura,
                "provincia": p,
                "radio_max": radio_max,
            }),
            None => json!({
                "calle": calle,
                "altura": altura,
                "radio_max": radio_max,
            }),
        })
        .await;

    let recomendaciones: Vec<RecomendacionPersonaVulnerable> = response.json();
    let recomendaciones = recomendaciones
        .iter()
        .map(|r| {
            (
                r.nombre.as_str(),
                r.apellido.as_str(),
                r.cantidad_recomendada,
            )
        })
        .collect::<Vec<(&str, &str, u16)>>();

    assert_eq!(recomendaciones_esperadas, recomendaciones);

    ctx.teardown().await;
}

#[rstest]
#[case(PersonaIn {
    personas: vec![PersonaInfo::new(
    "Pepe".to_string(),
    "Perez".to_string(),
    Direccion {
        provincia: "Buenos Aires".to_string(),
        calle: "CALLE 13".to_string(),
        altura: 45
    },
    vec![]
)]}, vec![PersonaOut::new(
    "Pepe".to_string(), 
    "Perez".to_string(), 
    Ubicacion {
        direccion: Direccion {
            provincia: "Buenos Aires".to_string(),
            calle: "CALLE 13".to_string(),
            altura: 45
        },
        coordenadas: Coordenadas { latitud: -34.842910871483994, longitud: -62.47609164477472 }
})])]
#[case(PersonaIn {
    personas: vec![
        PersonaInfo::new(
            "Pepe".to_string(),
            "Perez".to_string(),
            Direccion {
                provincia: "Buenos Aires".to_string(),
                calle: "CALLE 13".to_string(),
                altura: 45
            },
            vec![
                PersonaInfo::new(
                    "Pepa".to_string(),
                    "Perez".to_string(),
                    Direccion {
                        provincia: "Buenos Aires".to_string(),
                        calle: "CALLE 13".to_string(),
                        altura: 45
                    },
                    vec![]
                ),
                PersonaInfo::new(
                    "Pepito".to_string(),
                    "Perez".to_string(),
                    Direccion {
                        provincia: "Buenos Aires".to_string(),
                        calle: "CALLE 13".to_string(),
                        altura: 45
                    },
                    vec![]
                )
            ]
)]}, vec![PersonaOut {
    nombre: "Pepe".to_string(), 
    apellido: "Perez".to_string(), 
    direccion: Ubicacion {
        direccion: Direccion {
            provincia: "Buenos Aires".to_string(),
            calle: "CALLE 13".to_string(),
            altura: 45
        },
        coordenadas: Coordenadas { latitud: -34.842910871483994, longitud: -62.47609164477472 }
    }},
    PersonaOut {
        nombre: "Pepa".to_string(), 
        apellido: "Perez".to_string(), 
        direccion: Ubicacion {
            direccion: Direccion {
                provincia: "Buenos Aires".to_string(),
                calle: "CALLE 13".to_string(),
                altura: 45
            },
            coordenadas: Coordenadas { latitud: -34.842910871483994, longitud: -62.47609164477472 }
    }},
    PersonaOut {
        nombre: "Pepito".to_string(), 
        apellido: "Perez".to_string(), 
        direccion: Ubicacion {
            direccion: Direccion {
                provincia: "Buenos Aires".to_string(),
                calle: "CALLE 13".to_string(),
                altura: 45
            },
            coordenadas: Coordenadas { latitud: -34.842910871483994, longitud: -62.47609164477472 }
}}])]
#[tokio::test]
#[file_serial]
async fn test_endpoints_post_personas(
    #[case] personas: PersonaIn,
    #[case] resultado_esperado: Vec<PersonaOut>,
) {
    let ctx = TestContext::setup_with_migration().await;
    let (app, _) = setup_app(ctx.db.clone()).await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/personas_vulnerables")
        .json(&personas)
        .await;

    response.assert_json(&resultado_esperado);

    ctx.teardown().await;
}
