use crate::components::Header;
use crate::views::Instances;
use crate::error::Error;
use dioxus::prelude::*;
use dioxus::prelude::server_fn::error::NoCustomError;

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
                    let t = login().await;
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
pub async fn create() -> Result<(), ServerFnError> {
    use crate::backend::models::users::User;
    use crate::backend::AppState;
    use axum::Extension;
    use dioxus::prelude::server_fn::error::NoCustomError;

    let state: Extension<AppState> = extract().await?;
    let db = &state.db;

    let _ = User::create(&db, "test", "1234")
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}

#[server]
pub async fn login() -> Result<(), ServerFnError> {
    use crate::backend::models::users::User;
    use crate::backend::models::users::SESSION_USER_KEY;
    use crate::backend::AppState;
    use axum::Extension;
    use tower_sessions_core::Session;

    let state: Extension<AppState> = extract().await?;
    let db = &state.db;

    let user = User::login(db, "test", "1234").await?;

    let session: Session = extract()
        .await
        .map_err(|(_, message)| ServerFnError::new(message))?;
    session.insert(SESSION_USER_KEY, user.uuid).await?;

    Ok(())
}

#[server]
pub async fn check_login() -> Result<(), ServerFnError<Error>> {
    use crate::backend::models::users::User;

    let user: User = extract()
        .await?;
        // .map_err(|(_, message): (_, &str)| Error::Test(message.to_string()))?;
        // .map_err(|(_, message)| ServerFnError::new(message))?;
    println!("{}", user.username);

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::backend::models::users::SESSION_USER_KEY;
    use tower_sessions_core::Session;

    let session: Session = extract()
        .await
        .map_err(|(_, message)| ServerFnError::new(message))?;
    session.remove::<String>(SESSION_USER_KEY).await?;

    Ok(())
}
