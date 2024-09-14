use axum::{http::StatusCode, response::IntoResponse};

fn request_georef() 

pub fn get_recomendacion(
    calle: String,
    altura: String,
    provincia: Option<String>,
    radio_max: Option<i32>,
) -> impl IntoResponse {
    
    (StatusCode::ACCEPTED, "")
}
