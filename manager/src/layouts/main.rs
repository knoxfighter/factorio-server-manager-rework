use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn MainLayout() -> Element {
    let nav = navigator();

    let login = use_server_future(check_login)?;

    use_effect(move || {
        let val = login();
        // let v = login.value();
        // let v = v.unwrap();
        if val.is_some() && val.unwrap().is_err() {
            nav.push(Route::Login {});
        }
    });

    rsx! {
        Header {}

        Sidebar {}

        Outlet::<Route> {}
    }
}

#[get("/api/check_login", _user: crate::backend::models::users::User)]
pub async fn check_login() -> ServerFnResult {
    // use crate::backend::models::users::User;
    // let _: User = extract().await?;

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
