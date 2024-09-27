use super::test_utils::setup_app;
use crate::common::TestContext;
use axum_test::TestServer;
use rstest::rstest;
use serde_json::json;
use serial_test::file_serial;
use servicio_apiV2::routes::{
    heladeras::{HeladeraIn, HeladeraInfo, HeladeraOut, RecomendacionHeladera},
    Coordenadas, Direccion, Ubicacion,
};
use tokio;

#[rstest]
#[case("Medrano", 500, "CABA", 20.0, None, vec![
    (Coordenadas {latitud: -34.59, longitud: -58.42}, 10),
    (Coordenadas {latitud: -34.66, longitud: -58.41}, 30),
    (Coordenadas {latitud: -34.60, longitud: -58.44}, 34),
])]
#[case("Medrano", 500, "CABA", 20.0, Some(31), vec![
    (Coordenadas {latitud: -34.60, longitud: -58.44}, 34),
])]
#[tokio::test]
#[file_serial]
async fn test_heladeras_endpoints_recomendaciones(
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: &str,
    #[case] radio_max: f64,
    #[case] stock_minimo: Option<i16>,
    #[case] recomendaciones_esperadas: Vec<(Coordenadas, u16)>,
) {
    let ctx = TestContext::setup_with_migration().await;

    let (app, _) = setup_app(ctx.db.clone()).await;

    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/api/heladeras")
        .add_query_params(match stock_minimo {
            Some(s) => json!({
                "calle": calle,
                "altura": altura,
                "provincia": provincia,
                "radio_max": radio_max,
                "stock_minimo": s
            }),
            None => json!({
                "calle": calle,
                "altura": altura,
                "provincia": provincia,
                "radio_max": radio_max,
            }),
        })
        .await;

    let recomendaciones: Vec<RecomendacionHeladera> = response.json();
    let recomendaciones = recomendaciones
        .iter()
        .map(|r| (r.ubicacion.coordenadas, r.cantidad_recomendada))
        .collect::<Vec<(Coordenadas, u16)>>();

    assert_eq!(recomendaciones_esperadas, recomendaciones);

    ctx.teardown().await;
}

#[rstest]
#[case(HeladeraIn {
    heladeras: vec![HeladeraInfo::new(Direccion {
        provincia: "CABA".to_string(),
        calle: "Medrano".to_string(),
        altura: 300
    }, 12),
    HeladeraInfo::new(Direccion {
        provincia: "CABA".to_string(),
        calle: "San Juan".to_string(),
        altura: 30
    }, 14)
]
}, vec![HeladeraOut {
    ubicacion: Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
            calle: "AV MEDRANO".to_string(),
            altura: 300
        },
        coordenadas: Coordenadas {latitud: -34.6072678707556, longitud: -58.4214130064098 }
    },
    cantidad_viandas: 12
}, HeladeraOut {
    ubicacion: Ubicacion {
        direccion: Direccion {
            provincia: "Ciudad Autónoma de Buenos Aires".to_string(),
        calle: "AV SAN JUAN".to_string(),
        altura: 30
        },
        coordenadas: Coordenadas {latitud: -34.62152886414054, longitud: -58.36631330847936 }
    },
    cantidad_viandas: 14
}])]
#[tokio::test]
#[file_serial]
async fn test_endpoints_post_heladeras(
    #[case] heladeras: HeladeraIn,
    #[case] resultado_esperado: Vec<HeladeraOut>,
) {
    let ctx = TestContext::setup_with_migration().await;
    let (app, _) = setup_app(ctx.db.clone()).await;
    let server = TestServer::new(app).unwrap();

    let response = server.post("/api/heladeras").json(&heladeras).await;

    response.assert_json(&resultado_esperado);

    ctx.teardown().await;
}
