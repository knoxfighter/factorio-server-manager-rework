use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_primitives::select::*;

#[component]
pub fn NewInstance() -> Element {
    let fruits = ["Apple", "Banana", "Orange"];
    let fruits = fruits.iter().enumerate().map(|(index, fruit)| {
        rsx! {
            SelectOption::<Option<String>> {
                class: "select-option",
                index,
                value: *fruit,
                text_value: *fruit,
                {fruit.to_string()}
                SelectItemIndicator {
                    svg {
                        class: "select-check-icon",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M5 13l4 4L19 7" }
                    }
                }
            }
        }
    });

    let versions = use_server_future(get_factorio_versions)?;
    tracing::debug!("{:#?}", versions);

    rsx! {
        div {
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                },
                label {
                    "Name"
                    input { name: "Name" }
                }
                Select::<Option<String>> {
                    width: "12rem",
                    class: "select",
                    placeholder: "Select Factorio Version",
                    SelectTrigger {
                        class: "select-trigger",
                        width: "12rem",
                        aria_label: "Select Trigger",
                        SelectValue {}
                    }
                    SelectList { class: "select-list", aria_label: "Select Demo",
                        SelectGroup { class: "select-group",
                            SelectGroupLabel { class: "select-group-label", "Fruits" }
                            {fruits}
                        }
                        SelectGroup { class: "select-group",
                            SelectGroupLabel { class: "select-group-label", "Other" }
                            SelectOption::<Option<String>> {
                                class: "select-option",
                                index: 4usize,
                                value: None,
                                text_value: "other",
                                "Other"
                                SelectItemIndicator {
                                    svg {
                                        class: "select-check-icon",
                                        view_box: "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M5 13l4 4L19 7" }
                                    }
                                }
                            }
                        }
                    }
                }
                label {
                    "Factorio Version"
                    input { name: "Factorio Version" }
                }
                input { name: "Create", r#type: "submit", value: "Create" }
            }
        }
    }
}

#[server]
async fn get_factorio_versions() -> ServerFnResult<Vec<String>> {
    use crate::backend::AppState;
    use axum::Extension;
    
    let state: Extension<AppState> = extract().await?;

    let versions = state.manager.cache().get_available_versions().await?;
    
    Ok(versions.keys().map(|v| v.to_string()).collect())
}
