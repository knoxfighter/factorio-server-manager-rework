use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde::ser::Error;

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum ServerError {
    // #[error("test error: {0}")]
    Test(String),
}

pub(crate) fn map_server_error(err: impl Into<ServerError>) -> ServerError {
    err.into()
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).map_err(|e| std::fmt::Error::custom(e.to_string()))?;
        write!(f, "{}", s)
    }
}

impl FromStr for ServerError {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
