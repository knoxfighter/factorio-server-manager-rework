use crate::backend::config::Config;
use crate::backend::error::BackendError;
use crate::backend::models::users::User;
use crate::App;
use axum::Extension;
use diesel::SqliteConnection;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::AsyncMigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dioxus::logger::tracing::dispatcher::SetGlobalDefaultError;
use dioxus::logger::tracing::subscriber::set_global_default;
use dioxus::logger::tracing::Level;
use dioxus::prelude::{DioxusRouterExt, ServeConfig};
use factorio_server::manager::Manager;
use std::sync::Arc;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions_core::Expiry;

mod config;
pub mod error;
pub mod models;
pub mod schema;

pub type DbPool = Pool<SyncConnectionWrapper<SqliteConnection>>;
pub type DbPoolConnection<'a> =
    PooledConnection<'a, AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: Config,
    pub manager: Arc<Manager>,
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
    dotenvy::dotenv().ok();

    if cfg!(debug_assertions) {
        _ = init(Level::DEBUG);
    } else {
        _ = init(Level::INFO);
    }

    let config = Config::init().unwrap();

    let pool = establish_db_connection(&config).await;

    run_db_migrations(&pool).await.unwrap();

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

    let state = AppState {
        db: pool,
        manager: Arc::new(Manager::new(&config.manager_path).unwrap()),
        config,
    };

    let app = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), App)
        .layer(
            ServiceBuilder::new()
                .layer(session_manager)
                .layer(Extension(state)),
        );

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn establish_db_connection(config: &Config) -> DbPool {
    let manager = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(
        &config.database_file,
    );

    let pool: DbPool = Pool::builder().build(manager).await.unwrap();
    pool
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

async fn run_db_migrations(db: &DbPool) -> Result<(), BackendError> {
    let conn = db.get_owned().await?;
    let mut conn = AsyncMigrationHarness::new(conn);
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    Ok(())
}
