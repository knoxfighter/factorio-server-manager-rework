use crate::backend::error::BackendError::AxumRejection;
use crate::error::ServerError;
use argon2::password_hash::rand_core::OsError;
use argon2::password_hash::Error;
use axum::body::Body;
use axum::extract::rejection::ExtensionRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel_async::pooled_connection::bb8::RunError;
use dioxus::logger::tracing;
use thiserror::Error;

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
}

impl From<BackendError> for ServerError {
    fn from(value: BackendError) -> Self {
        tracing::info!("backend error converted: {:?}", value);
        ServerError::Test("converted BackendError".to_string())
    }
}

macro_rules! to_server_error_via_backend_error {
    ($t:ty) => {
        impl From<$t> for ServerError {
            fn from(value: $t) -> Self {
                BackendError::from(value).into()
            }
        }
    };
    ($($t:ty),*) => {
        $(
            to_server_error_via_backend_error!($t);
        )*
    };
}

to_server_error_via_backend_error!(
    diesel::result::Error,
    RunError,
    argon2::Error,
    OsError,
    argon2::password_hash::Error,
    ExtensionRejection,
    (StatusCode, &str),
    tower_sessions::session::Error
);

impl From<(StatusCode, &str)> for BackendError {
    fn from(value: (StatusCode, &str)) -> Self {
        AxumRejection(value.0, value.1.to_string())
    }
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        // This code is not called when using dioxus internal systems.
        // It gets converted to a ServerFnError and then to a Response.

        tracing::info!("IntoResponse for BackendError");
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}
