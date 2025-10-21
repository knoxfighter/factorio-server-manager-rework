use dioxus::logger::tracing;
use dioxus::prelude::*;
use factorio_version::Version;
use crate::components::select::*;

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
                SelectItemIndicator {}
            }
        }
    });

    let versions = use_server_future(get_factorio_versions)?;
    let mut versions = versions().unwrap()?;
    versions.sort_by(|a, b| b.cmp(a));
    let versions = versions.iter().enumerate().map(|(index, version)| {
        rsx! {
            SelectOption::<String> {
                class: "select-option",
                index,
                value: version.to_string(),
                text_value: version.to_string(),
                {version.to_string()}
                SelectItemIndicator {}
            }
        }
    });

    let mut selected_version = use_signal(|| "stable".to_string());

    rsx! {
        div {
            form {
                onsubmit: move |event: Event<FormData>| {
                    event.prevent_default();
                    tracing::info!("submit: {event:?} - {selected_version}");
                    let t = match event.get_first("name").unwrap() {
                        FormValue::Text(t) => Ok(t),
                        FormValue::File(_) => Err("test".to_string()),
                    }
                        .unwrap();
                    Ok(())
                },
                label {
                    "Name"
                    input { name: "name" }
                }
                Select::<String> {
                    width: "12rem",
                    class: "select",
                    placeholder: "Select Factorio Version",
                    name: "version",
                    default_value: selected_version(),
                    on_value_change: move |value: Option<String>| {
                        selected_version.set(value.unwrap());
                    },
                    SelectTrigger {
                        class: "select-trigger",
                        width: "12rem",
                        aria_label: "Select Trigger",
                        SelectValue {}
                    }
                    SelectList { class: "select-list", aria_label: "Select Version",
                        // TODO: add current latest version to string
                        SelectGroup { class: "select-group",
                            // SelectGroupLabel { class: "select-group-label", "Other" }
                            SelectOption::<String> {
                                class: "select-option",
                                index: versions.len(),
                                value: "latest",
                                text_value: "latest",
                                "latest"
                                SelectItemIndicator {}
                            }
                            // TODO: add current stable version to string
                            SelectOption::<String> {
                                class: "select-option",
                                index: versions.len(),
                                value: "stable",
                                text_value: "stable",
                                "stable"
                                SelectItemIndicator {}
                            }
                        }
                        SelectGroup { class: "select-group",
                            // SelectGroupLabel { class: "select-group-label", "Fruits" }
                            {versions}
                        }
                    }
                }
                input { name: "Create", r#type: "submit", value: "Create" }
            }
        }
    }
}

#[get("/api/factorio_versions", state: axum::Extension<crate::backend::AppState>)]
async fn get_factorio_versions() -> ServerFnResult<Vec<Version>> {
    use crate::backend::error::BackendError;

    let versions = state
        .manager
        .cache()
        .get_available_versions()
        .await
        .map_err(BackendError::from)?;

    Ok(versions.keys().map(|v| (*v).into()).collect())
}
