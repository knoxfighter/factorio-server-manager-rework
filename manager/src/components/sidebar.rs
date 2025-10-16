use crate::Route;
use dioxus::prelude::*;

const INSTANCES_CSS: Asset = asset!("/assets/styling/instances.css");

#[component]
pub fn Sidebar() -> Element {
    let items = use_signal(|| vec!["Instance1", "Instance2", "Instance3"]);

    rsx! {
        document::Link { rel: "stylesheet", href: INSTANCES_CSS }

        div { id: "instances",
            div { class: "list",
                ul {
                    for item in items.iter() {
                        li { "{item}" }
                    }
                }
            }

            div { class: "controls",
                ul {
                    li {
                        Link { to: Route::NewInstance {}, "New Instance" }
                    }
                    li { "Settings" }
                }
            }
        }
    }
}
