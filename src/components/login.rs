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
                event.get_first("username").map_or(Default::default(), |e| match e {
                    FormValue::Text(t) => t,
                    FormValue::File(_) => Default::default(),
                }),
                event.get_first("password").map_or(Default::default(), |e| match e {
                    FormValue::Text(t) => t,
                    FormValue::File(_) => Default::default(),
                }).into(),
                // event.values()["username"].as_value(),
                // event.values()["password"].as_value().into(),
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

#[post("/api/login", session: tower_sessions_core::Session, state: axum::Extension<crate::backend::AppState>)]
pub async fn login(username: String, password: Password) -> ServerFnResult {
    use crate::backend::models::users::User;
    use crate::backend::models::users::SESSION_USER_KEY;
    use crate::backend::error::BackendError;

    let db = &state.db;

    let user = User::login(db, username, password).await?;

    session.insert(SESSION_USER_KEY, user.uuid).await.map_err(BackendError::from)?;

    Ok(())
}
