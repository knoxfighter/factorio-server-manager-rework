use crate::components::Header;
use crate::views::Instances;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn MainLayout() -> Element {
    let nav = navigator();

    let login = use_server_future(check_login)?;
    let v = login.value();
    let v = v.unwrap();
    if let Err(_) = v {
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
// pub async fn create() -> Result<(), ServerError> {
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
