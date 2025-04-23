use dioxus::prelude::*;

mod components;
mod layouts;
mod views;

#[cfg(feature = "server")]
mod backend;
mod error;

use layouts::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/factorio-wheel.png");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const NORMALIZE: Asset = asset!("/assets/styling/normalize.css");

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    backend::launch().await;
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: NORMALIZE }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
