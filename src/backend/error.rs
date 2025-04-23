use diesel_async::pooled_connection::bb8::RunError;
use thiserror::Error;
use crate::error::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Diesel connection error: {0}")]
    PoolError(#[from] RunError),

    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::Error),

    #[error("Password Hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
}

impl Into<Error> for ServerError {
    fn into(self) -> Error {
        Error::Test(self.to_string())
    }
}
