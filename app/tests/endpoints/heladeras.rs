use rstest::rstest;
use tokio;

#[rstest]
#[case("/personas_vulnerables", "Urquiza", 400, Some("Santa Fe"), 20.0, None)]
#[tokio::test]
async fn test_endpoints(
    #[case] endpoint: &str,
    #[case] calle: &str,
    #[case] altura: i16,
    #[case] provincia: Option<&str>,
    #[case] radio_max: f64,
    #[case] stock_minimo: Option<i16>,
) {
}
