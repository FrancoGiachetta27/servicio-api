use sea_orm::MockDatabase;
use test_case::case;
use tokio;

async fn setup() {
    let db = MockDatabase::new(sea_orm::DatabaseBackend::MySql).append_query_results(
        [
            
        ]
    )
}

#[tokio::test]
#[test_case("/personas_vulnerables", "Urquiza", 400, Some("Santa Fe"), 20.0, None)]
async fn test_endpoints(
    endpoint: &str,
    calle: &str,
    altura: i16,
    provincia: Option<&str>,
    radio_max: f64,
    stock_minimo: Option<i16>
) {
    
}
