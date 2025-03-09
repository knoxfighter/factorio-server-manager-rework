use std::env;
use diesel::SqliteConnection;
use dioxus_fullstack::prelude::DioxusRouterExt;
use dioxus_fullstack::ServeConfig;
use crate::{App};

pub mod models;
pub mod schema;

pub async fn launch() {
    dioxus::logger::initialize_default();

    let connection = establish_db_connection();

    let addr = dioxus::cli_config::fullstack_address_or_localhost();

    let app = axum::Router::new().serve_dioxus_application(ServeConfig::new().unwrap(), App).into_make_service();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn establish_db_connection() -> SqliteConnection {
    use diesel::Connection;

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
