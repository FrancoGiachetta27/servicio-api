use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub enum AppError {
    IoError(std::io::Error),
    JsonRejection(JsonRejection),
    ServiceError(ureq::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorMessage {
            message: String,
        }

        let (status, message) = match self {
            // though the io::Error is quite general, it is only used
            // when parsing json by ureq
            AppError::IoError(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("No se pudo deserializar la respusta: {}", e),
            ),
            AppError::JsonRejection(e) => (e.status(), e.body_text()),
            AppError::ServiceError(e) => match e {
                ureq::Error::Status(code, resp) => {
                    // as the error refers to the status, it's fine to
                    // just unwrap the value
                    let status = StatusCode::from_u16(code).unwrap();
                    (status, resp.into_string().unwrap())
                }
                ureq::Error::Transport(e) => (
                    StatusCode::FAILED_DEPENDENCY,
                    match e.message() {
                        Some(m) => m.to_string(),
                        None => {
                            format!("Error al recibir la repuesta del url {}", e.url().unwrap())
                        }
                    },
                ),
            },
        };

        (status, Json(ErrorMessage { message })).into_response()
    }
}

impl From<ureq::Error> for AppError {
    fn from(value: ureq::Error) -> Self {
        Self::ServiceError(value)
    }
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
