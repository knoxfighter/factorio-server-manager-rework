use crate::components::Echo;
use crate::components::Login;
use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;

mod components;
mod layouts;
mod views;

#[cfg(feature = "server")]
mod backend;
mod error;

use layouts::MainLayout;
use layouts::RootLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(RootLayout)]
        #[route("/login")]
        Login {},

        #[layout(MainLayout)]
            #[route("/")]
            Echo {},
            // #[route("/blog/:id")]
            // Blog { id: i32 },
}

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
    rsx! {
        ToastProvider { Router::<Route> {} }
    }
}
