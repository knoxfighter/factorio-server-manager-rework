use crate::backend::error::BackendError::{AxumRejection, SessionError};
use axum::body::Body;
use axum::extract::rejection::ExtensionRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel_async::pooled_connection::bb8::RunError;
use dioxus::logger::tracing;
use dioxus::server::ServerFnError;
use thiserror::Error;

// TODO: Adjust strings so they can be shown to the user
#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Diesel connection error: {0}")]
    PoolError(#[from] RunError),

    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),

    #[error("OsRng error: {0}")]
    OsRngError(#[from] argon2::password_hash::rand_core::OsError),

    #[error("Password Hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("Axum Extension Rejection error: {0}")]
    AxumExtensionRejection(#[from] ExtensionRejection),

    #[error("Axum Rejection error: {0} - {1}")]
    AxumRejection(StatusCode, String),

    #[error("Session error: {0}")]
    SessionError(#[from] tower_sessions::session::Error),

    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Factorio Server Crate Error: {0}")]
    FactorioServerCrateError(#[from] factorio_server::error::ServerError),
}

impl From<(StatusCode, &str)> for BackendError {
    fn from(value: (StatusCode, &str)) -> Self {
        AxumRejection(value.0, value.1.to_string())
    }
}

impl From<BackendError> for ServerFnError {
    fn from(value: BackendError) -> Self {
        ServerFnError::ServerError {
            message: value.to_string(),
            code: StatusCode::from(value).as_u16(),
            details: None,
        }
    }
}

impl From<BackendError> for StatusCode {
    fn from(value: BackendError) -> Self {
        match value {
            AxumRejection(code, _) => code,
            SessionError(_) => StatusCode::UNAUTHORIZED,
            BackendError::PasswordHashError(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        // This code is not called when using dioxus internal systems.
        // It gets converted to a ServerFnError and then to a Response.

        tracing::error!("IntoResponse for BackendError");
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}
