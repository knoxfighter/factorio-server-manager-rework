use crate::Route;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};
use std::time::Duration;

const HEADER: Asset = asset!("/assets/styling/header.css");
const ICON: Asset = asset!("/assets/factorio-wheel.png");

#[component]
pub fn Header() -> Element {
    let nav = use_navigator();
    let mut toast_api = use_toast();

    rsx! {
        document::Link { rel: "stylesheet", href: HEADER }

        div { id: "header",
            img { src: ICON }

            div {
                button {
                    onclick: move |_| async move {
                        let r = logout().await;
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

#[server]
pub async fn logout() -> ServerFnResult {
    use crate::backend::error::BackendError;
    use crate::backend::models::users::SESSION_USER_KEY;
    use tower_sessions_core::Session;

    let session: Session = extract().await.map_err(|e| BackendError::from(e))?;
    tracing::info!("Logging out: {:?}", session);
    session.remove::<String>(SESSION_USER_KEY).await?;

    Ok(())
}
