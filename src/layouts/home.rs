use dioxus::prelude::*;
use crate::components::Header;
use crate::views::Instances;

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}

        Instances {}
    }
}
