use rtest::*;
use tokio;

fn setup() {}

#[tokio::test]
#[rtest]
#[case("/personas_vulnerables", "Urquiza", 400, "Santa Fe", 20.0, None)]
async fn test_endpoints(
    #[case] endpoint: &'static str,
    #[case] calle: &'static str,
    #[case] altura: i16,
    #[case] provincia: Option<&'static str>,
    #[case] radio_max: f64,
    #[case] stock_minimo: Option<i16>,
) {
}
