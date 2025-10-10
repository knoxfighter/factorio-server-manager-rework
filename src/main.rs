use crate::components::echo::Echo;
use crate::components::login::Login;
use crate::views::instances::new_instance::NewInstance;
use dioxus::prelude::*;
use dioxus_primitives::toast::ToastProvider;
use layouts::main::MainLayout;
use layouts::root::RootLayout;

mod components;
mod layouts;
mod views;
#[cfg(feature = "server")]
mod backend;
pub mod password;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(RootLayout)]
        #[route("/login")]
        Login {},

        #[layout(MainLayout)]
            #[route("/")]
            Echo {},

            #[route("/instances/new")]
             NewInstance {},
            // #[route("/blog/:id")]
            // Blog { id: i32 },
        // #[end_layout]
    // #[end_layout]
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
