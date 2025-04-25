use dioxus::logger::tracing;
use crate::components::Header;
use crate::views::Instances;
use dioxus::prelude::*;
use crate::error::{map_server_error, ServerError};

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}

        Instances {}

        div {
            button {
                id: "create-button",
                onclick: move |_| async move {
                    let t = create().await;
                },
                "create"
            }

            button {
                id: "test-button2",
                onclick: move |_| async move {
                    let t = login().await/*.map_err(|err| CapturedError::from_display(err))*/;
                    tracing::info!("test: {}", t.is_ok());
                    tracing::info!("{:?}", t);
                    Ok(())
                },
                "login"
            }

            button {
                id: "test-button3",
                onclick: move |_| async move {
                    let t = check_login().await;
                },
                "check"
            }

            button {
                id: "test-button4",
                onclick: move |_| async move {
                    let t = logout().await;
                },
                "logout"
            }
        }
    }
}

#[server]
pub async fn create() -> Result<(), ServerFnError<ServerError>> {
    use crate::backend::models::users::User;
    use crate::backend::AppState;
    use axum::Extension;
    use crate::backend::error::map_backend_error;

    let state: Extension<AppState> = extract().await.map_err(map_backend_error)?;
    let db = &state.db;
    
    let _ = User::create(&db, "test", "1234")
        .await.map_err(map_server_error)?;

    Ok(())
}

#[server]
pub async fn login() -> Result<(), ServerFnError<ServerError>> {
    use crate::backend::models::users::User;
    use crate::backend::models::users::SESSION_USER_KEY;
    use crate::backend::AppState;
    use axum::Extension;
    use tower_sessions_core::Session;
    use crate::backend::error::map_backend_error;

    let state: Extension<AppState> = extract().await.map_err(map_backend_error)?;
    let db = &state.db;

    let user = User::login(db, "test", "1234").await.map_err(map_server_error)?;

    let session: Session = extract()
        .await.map_err(map_backend_error)?;
    session.insert(SESSION_USER_KEY, user.uuid).await.map_err(map_backend_error)?;

    Ok(())
}

#[server]
pub async fn check_login() -> Result<(), ServerFnError<ServerError>> {
    use crate::backend::models::users::User;
    use crate::backend::error::map_backend_error;

    let user: User = extract()
        .await.map_err(map_backend_error)?;
    println!("{}", user.username);

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError<ServerError>> {
    use crate::backend::models::users::SESSION_USER_KEY;
    use tower_sessions_core::Session;
    use crate::backend::error::map_backend_error;

    let session: Session = extract()
        .await.map_err(map_backend_error)?;
    session.remove::<String>(SESSION_USER_KEY).await.map_err(map_backend_error)?;

    Ok(())
}
