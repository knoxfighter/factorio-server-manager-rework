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
                event.values()["password"].as_value(),
            )
            .await
            {
                Ok(_) => {
                    navigator.push("/");
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
pub async fn login(username: String, password: String) -> ServerFnResult {
    use crate::backend::error::BackendError;
    use crate::backend::models::users::User;
    use crate::backend::models::users::SESSION_USER_KEY;
    use crate::backend::AppState;
    use axum::Extension;
    use tower_sessions_core::Session;

    let state: Extension<AppState> = extract().await?;
    let db = &state.db;

    let user = User::login(db, username, password).await?;

    let session: Session = extract().await.map_err(|e| BackendError::from(e))?;
    session.insert(SESSION_USER_KEY, user.uuid).await?;

    Ok(())
}
