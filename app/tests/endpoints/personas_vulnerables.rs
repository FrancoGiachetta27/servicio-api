use axum_test::TestServer;
use rstest::rstest;
use serde_json::json;
use servicio_apiV2::routes::personas_vulnerables::RecomendacionPersonaVulnerable;
use tokio;

use super::test_utils::setup_app;

#[rstest]
#[case("personas_vulnerables", "Medrano", 500, Some("CABA"), 20.0, vec![
    ("Maria", "Perez", 1),
    ("Nicole", "Perez", 1),
    ("Florencia", "Perez", 1)
])]
#[tokio::test]
async fn test_endpoints(
    #[case] endpoint: &str,
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: Option<&str>,
    #[case] radio_max: f64,
    #[case] recomendaciones_esperadas: Vec<(&str, &str, u16)>,
) {
    let app = setup_app().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .get(&format!("/api/{}", endpoint.to_string()))
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
