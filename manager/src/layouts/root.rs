use crate::Route;
use dioxus::core::Element;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_primitives::toast::{use_toast, ToastOptions};

static FAVICON: Asset = asset!("/assets/factorio-wheel.png");
static NORMALIZE: Asset = asset!("/assets/styling/normalize.css");
static MAIN_CSS: Asset = asset!("assets/styling/main.css");
static COMPONENTS_CSS: Asset = asset!("assets/dx-components-theme.css");

#[component]
pub fn RootLayout() -> Element {
    let toast_api = use_toast();

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        Stylesheet { href: NORMALIZE }
        Stylesheet { href: MAIN_CSS }
        Stylesheet { href: COMPONENTS_CSS }

        ErrorBoundary {
            handle_error: move |err: ErrorContext| {
                err.error()
                    .iter()
                    .for_each(|e| {
                        toast_api.error(e.to_string(), ToastOptions::new());
                        tracing::error!("{}", e.to_string());
                    });
                err.clear_errors();
                rsx! {
                    "If you see this something is majorly wrong!"
                    "Please report this error and tell us how you got here."
                }
            },
            Outlet::<Route> {}
        }
    }
}
