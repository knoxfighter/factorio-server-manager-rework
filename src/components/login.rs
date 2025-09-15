use crate::password::Password;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut login_error: Signal<Option<String>> = use_signal(|| None);

    let navigator = use_navigator();

    let onsubmit = move |event: FormEvent| {
        event.prevent_default();

        spawn(async move {
            match login(
                event.values()["username"].as_value(),
                event.values()["password"].as_value().into(),
            )
            .await
            {
                Ok(_) => {
                    navigator.push(Route::Echo {});
                }
                Err(err) => {
                    login_error.set(Some(err.to_string()));
                }
            }
        });
    };

    rsx! {
        div {
            form { onsubmit, action: "/login", method: "post",

                input {
                    r#type: "text",
                    placeholder: "Username",
                    name: "username",
                    id: "username",
                }

                input {
                    r#type: "password",
                    placeholder: "Password",
                    name: "password",
                    id: "password",
                }

                input { r#type: "submit", value: "Login" }

                if let Some(err) = login_error() {
                    div { "{err}" }
                }
            }
        }
    }
}

#[server(LoginLogin)]
pub async fn login(username: String, password: Password) -> ServerFnResult {
    use crate::backend::error::BackendError;
    use crate::backend::models::users::User;
    use crate::backend::models::users::SESSION_USER_KEY;
    use crate::backend::AppState;
    use axum::http::StatusCode;
    use axum::Extension;
    use dioxus::fullstack::server_context;
    use tower_sessions_core::Session;

    let ctx = server_context();
    let mut p = ctx.status_mut();
    *p = StatusCode::BAD_GATEWAY;

    let state: Extension<AppState> = extract().await?;
    let db = &state.db;

    let user = User::login(db, username, password).await?;

    let session: Session = extract().await.map_err(BackendError::from)?;
    session.insert(SESSION_USER_KEY, user.uuid).await?;

    Ok(())
}
