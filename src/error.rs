use dioxus::logger::tracing;
use dioxus::prelude::server_fn::codec::JsonEncoding;
use dioxus::prelude::server_fn::error::{FromServerFnError, ServerFnErrorErr};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum ServerError {
    #[error("test error: {0}")]
    Test(String),

    #[error("server fn error: {0}")]
    ServerFnError(ServerFnErrorErr),
}

impl FromServerFnError for ServerError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        tracing::info!("from_server_fn_error: {:?}", value);
        Self::ServerFnError(value)
    }
}
