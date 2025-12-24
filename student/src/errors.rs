use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub enum Error {
    InternalServerError,
    StudentNotFoundError,
    SchoolNotFoundError,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Error::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Server Error."),
            Error::StudentNotFoundError => (StatusCode::NOT_FOUND, "Student not found Error."),
            Error::SchoolNotFoundError => (StatusCode::NOT_FOUND, "School not found Error."),
        };

        return (
            status,
            Json(ErrorResponse {
                error: message.to_string(),
            }),
        )
            .into_response();
    }
}
