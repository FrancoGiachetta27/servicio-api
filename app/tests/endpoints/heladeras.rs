use test_case::case;
use tokio;

fn setup() {}

#[tokio::test]
#[test_case("/personas_vulnerables", "Urquiza", 400, Some("Santa Fe"), 20.0, None)]
async fn test_endpoints(
    endpoint: &'static str,
    calle: &'static str,
    altura: i16,
    provincia: Option<&'static str>,
    radio_max: f64,
    stock_minimo: Option<i16>,
) {
}
