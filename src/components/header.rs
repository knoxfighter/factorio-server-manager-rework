use dioxus::prelude::*;

const HEADER: Asset = asset!("/assets/styling/header.css");
const ICON: Asset = asset!("/assets/factorio-wheel.png");

#[component]
pub fn Header() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HEADER }

        div {
            id: "header",

            img {
                src: ICON
            }
        }
    }
}
