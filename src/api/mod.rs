use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub mod post;
pub mod thread;

pub enum Error {
    Database(sea_orm::error::DbErr),
    Render,
    ThreadNotFound,
    BoardNotFound,
}

impl IntoResponse for Error {
    // TODO! сделать страницу с красивым сообщением
    fn into_response(self) -> Response {
        match self {
            Error::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error\n{err}"),
            )
                .into_response(),
            Error::Render => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Page not rendered").into_response()
            }
            Error::ThreadNotFound => (StatusCode::NOT_FOUND, "Thread not found").into_response(),
            Error::BoardNotFound => (StatusCode::NOT_FOUND, "Board not found").into_response(),
        }
    }
}
