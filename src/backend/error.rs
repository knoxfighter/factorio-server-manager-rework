use axum::extract::rejection::ExtensionRejection;
use axum::http::StatusCode;
use diesel_async::pooled_connection::bb8::RunError;
use dioxus::logger::tracing;
use thiserror::Error;
use crate::backend::error::BackendError::AxumRejection;
use crate::error::ServerError;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Diesel connection error: {0}")]
    PoolError(#[from] RunError),

    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),

    #[error("Password Hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("Axum Extension Rejection error: {0}")]
    AxumExtensionRejection(#[from] ExtensionRejection),
    
    #[error("Axum Rejection error: {0}")]
    AxumRejection(String),

    #[error("Session error: {0}")]
    SessionError(#[from] tower_sessions::session::Error),
}

impl Into<ServerError> for BackendError {
    fn into(self) -> ServerError {
        tracing::info!("backend error converted: {:?}", self);
        ServerError::Test("converted BackendError".to_string())
    }
}

pub(crate) fn map_backend_error(err: impl Into<BackendError>) -> ServerError {
    err.into().into()
}

impl From<(StatusCode, &str)> for BackendError {
    fn from(value: (StatusCode, &str)) -> Self {
        AxumRejection(value.1.to_string())
    }
}
