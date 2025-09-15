use crate::components::header::Header;
use crate::views::instances::Instances;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn MainLayout() -> Element {
    let nav = navigator();

    let login = use_server_future(check_login)?;
    let v = login.value();
    let v = v.unwrap();
    if v.is_err() {
        nav.push(Route::Login {});
    }

    rsx! {
        Header {}

        Instances {}

        Outlet::<Route> {}
    }
}

#[server]
pub async fn check_login() -> ServerFnResult {
    use crate::backend::models::users::User;
    let _: User = extract().await?;

    Ok(())
}

// #[server]
// pub async fn create() -> Result<(), ServerFnError> {
//     use crate::backend::models::users::User;
//     use crate::backend::AppState;
//     use axum::Extension;
//
//     let state: Extension<AppState> = extract().await?;
//     let db = &state.db;
//
//     let _ = User::create(&db, "test", "1234")
//         .await?;
//
//     Ok(())
// }
