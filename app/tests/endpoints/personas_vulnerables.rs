use axum_test::TestServer;
use migration::{Migrator, MigratorTrait};
use rstest::rstest;
use serde_json::json;
use servicio_apiV2::routes::{
    personas_vulnerables::{PersonaIn, PersonaOut, RecomendacionPersonaVulnerable},
    Coordenadas, Direccion, Ubicacion,
};
use tokio;

use super::test_utils::setup_app;

#[rstest]
#[case("Medrano", 500, Some("CABA"), 10.0, vec![
    ("Maria", "Perez", 1),
    ("Nicole", "Perez", 1),
    ("Florencia", "Perez", 1)
])]
#[tokio::test]
async fn test_endpoints_recomendaciones(
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: Option<&str>,
    #[case] radio_max: f64,
    #[case] recomendaciones_esperadas: Vec<(&str, &str, u16)>,
) {
    let (app, _) = setup_app().await;
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
}

#[rstest]
#[case(vec![PersonaIn {
    nombre: "Pepe".to_string(),
    apellido: "Perez".to_string(),
    direccion: Direccion {
        provincia: "CABA".to_string(),
        calle: "Balcarce".to_string(),
        altura: 78
    },
    hijos: None
}], vec![PersonaOut::new(
    "Pepe".to_string(), 
    "Perez".to_string(), 
    Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
            calle: "BALCARCE".to_string(),
            altura: 78
        },
        coordenadas: Coordenadas { latitud: -34.60856585458922, longitud: -58.37072506895037 }
})])]
#[case(vec![PersonaIn {
    nombre: "Pepe".to_string(),
    apellido: "Perez".to_string(),
    direccion: Direccion {
        provincia: "CABA".to_string(),
        calle: "Mozart".to_string(),
        altura: 2300
    },
    hijos: Some(vec![
        PersonaIn {
            nombre: "Pepa".to_string(),
            apellido: "Perez".to_string(),
            direccion: Direccion {
                provincia: "CABA".to_string(),
                calle: "Mozart".to_string(),
        altura: 2300
            },
            hijos: None
        },
        PersonaIn {
            nombre: "Pepito".to_string(),
            apellido: "Perez".to_string(),
            direccion: Direccion {
                provincia: "CABA".to_string(),
                calle: "Mozart".to_string(),
                altura: 2300
            },
            hijos: None
        }
    ])
}], vec![PersonaOut {
    nombre: "Pepe".to_string(), 
    apellido: "Perez".to_string(), 
    direccion: Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
            calle: "MOZART".to_string(),
            altura: 2300
        },
        coordenadas: Coordenadas { latitud: -34.6594301720661, longitud: -58.4690932428077 }
}},
PersonaOut {
    nombre: "Pepa".to_string(), 
    apellido: "Perez".to_string(), 
    direccion: Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
            calle: "MOZART".to_string(),
            altura: 2300
        },
        coordenadas: Coordenadas { latitud: -34.6594301720661, longitud: -58.4690932428077 }
}},PersonaOut {
    nombre: "Pepito".to_string(), 
    apellido: "Perez".to_string(), 
    direccion: Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
            calle: "MOZART".to_string(),
            altura: 2300
        },
        coordenadas: Coordenadas { latitud: -34.6594301720661, longitud: -58.4690932428077 }
}}])]
#[tokio::test]
async fn test_endpoints_post_personas(
    #[case] personas: Vec<PersonaIn>,
    #[case] resultado_esperado: Vec<PersonaOut>,
) {
    let (app, db) = setup_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/personas_vulnerables")
        .json(&personas)
        .await;
    let resultado: Vec<PersonaOut> = response.json();

    assert_eq!(resultado_esperado, resultado);

    Migrator::down(&db, None).await.ok();
}
