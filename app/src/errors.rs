use axum::{
    extract::rejection::JsonRejection,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use sea_orm::DbErr;
use serde::Serialize;

pub enum AppError {
    IoError(std::io::Error),
    JsonRejection(JsonRejection),
    ServiceError(ureq::Error),
    DdError(DbErr),
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
            AppError::DdError(e) => (StatusCode::FAILED_DEPENDENCY, e.to_string()),
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

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        Self::DdError(value)
    }
}

pub async fn handle_404(uri: Uri) -> impl IntoResponse {
    #[derive(Serialize)]
    struct ErrorMessage {
        status: u16,
        message: String,
    }

    let error = ErrorMessage {
        status: StatusCode::NOT_FOUND.as_u16(),
        message: format!("EndPoint incorreto: {}", uri.path()),
    };

    (StatusCode::NOT_FOUND, Json(error))
}
