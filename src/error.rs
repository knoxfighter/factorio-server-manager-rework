use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    #[error("test error: {0}")]
    Test(String),
}

impl FromStr for Error {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if s.starts_with("test error: ") {
        //
        // }
        // todo!()

        Ok(Self::Test(s.to_string()))
    }
}
