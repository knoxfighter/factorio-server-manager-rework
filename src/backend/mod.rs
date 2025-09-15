use crate::backend::models::users::User;
use crate::App;
use axum::Extension;
use diesel::SqliteConnection;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use dioxus::logger::tracing::dispatcher::SetGlobalDefaultError;
use dioxus::logger::tracing::subscriber::set_global_default;
use dioxus::logger::tracing::Level;
use dioxus::prelude::DioxusRouterExt;
use dioxus_fullstack::ServeConfig;
use std::env;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions_core::Expiry;
pub mod error;
pub mod models;
pub mod schema;

pub type DbPool = Pool<SyncConnectionWrapper<SqliteConnection>>;
pub type DbPoolConnection<'a> =
    PooledConnection<'a, AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>>>;
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

// This function is an adjusted function from dioxus
pub fn init(level: Level) -> Result<(), SetGlobalDefaultError> {
    let sub = tracing_subscriber::FmtSubscriber::builder().with_env_filter(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(level.into())
            .from_env_lossy(),
    );

    if !dioxus_cli_config::is_cli_enabled() {
        return set_global_default(sub.finish());
    }

    // todo(jon): this is a small hack to clean up logging when running under the CLI
    // eventually we want to emit everything as json and let the CLI manage the parsing + display
    set_global_default(sub.without_time().with_target(false).finish())
}

pub async fn launch() {
    dotenvy::dotenv().unwrap();

    if cfg!(debug_assertions) {
        _ = init(Level::DEBUG);
    } else {
        _ = init(Level::INFO);
    }

    let pool = establish_db_connection().await;

    // assure that at least one user exists
    User::assure_admin_user(&pool).await.unwrap();

    let session_store = MemoryStore::default();

    // tokio::task::spawn(
    //     session_store
    //         .clone()
    //         .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    // );

    let key = tower_sessions::cookie::Key::generate();
    let session_manager = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)))
        .with_signed(key);

    let addr = dioxus::cli_config::fullstack_address_or_localhost();

    let state = AppState { db: pool };

    let app = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .layer(
            ServiceBuilder::new()
                .layer(session_manager)
                .layer(Extension(state)),
        );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn establish_db_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager =
        AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(&database_url);
    let pool: DbPool = Pool::builder().build(manager).await.unwrap();
    pool
}
