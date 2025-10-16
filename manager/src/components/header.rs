use crate::Route;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

const HEADER: Asset = asset!("/assets/styling/header.css");
const ICON: Asset = asset!("/assets/factorio-wheel.png");

#[component]
pub fn Header() -> Element {
    let nav = use_navigator();
    let toast_api = use_toast();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER }

        div { id: "header",
            img { src: ICON }

            div {
                button {
                    onclick: move |_| async move {
                        if let Err(e) = logout().await {
                            toast_api
                                .error(
                                    e.to_string(),
                                    ToastOptions::new().duration(Duration::from_secs(15)),
                                );
                        } else {
                            nav.push(Route::Login {});
                        }
                    },
                    "Logout"
                }
            }
        }
    }
}

#[post("/api/logout", session: tower_sessions_core::Session)]
pub async fn logout() -> ServerFnResult {
    use crate::backend::error::BackendError;
    use crate::backend::models::users::SESSION_USER_KEY;

    session
        .remove::<String>(SESSION_USER_KEY)
        .await
        .map_err(BackendError::from)?;

    Ok(())
}
