use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub mod post;
pub mod thread;

pub enum Error {
    DbErr, // (sea_orm::error::DbErr)
    ThreadNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DbErr => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            Error::ThreadNotFound => (StatusCode::NOT_FOUND, "Thread not found").into_response(),
        }
    }
}
