use axum_test::TestServer;
use rstest::rstest;
use serde_json::json;
use tokio;

use super::test_utils::setup_app;

#[rstest]
#[case("personas_vulnerables", "Urquiza", 400, Some("Santa Fe"), 20.0)]
#[tokio::test]
async fn test_endpoints(
    #[case] endpoint: &str,
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: Option<&str>,
    #[case] radio_max: f64,
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

    dbg!(response);
}
