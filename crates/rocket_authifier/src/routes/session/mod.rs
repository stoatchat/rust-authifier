use rocket::Route;
use crate::SecurityAddon;

pub mod edit;
pub mod fetch_all;
pub mod login;
pub mod logout;
pub mod revoke;
pub mod revoke_all;

#[derive(OpenApi)]
#[openapi(
    paths(
        login::login,
        logout::logout,
        fetch_all::fetch_all,
        revoke::revoke,
        revoke_all::revoke_all,
        edit::edit
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![
        login::login,
        logout::logout,
        fetch_all::fetch_all,
        revoke::revoke,
        revoke_all::revoke_all,
        edit::edit
    ]
}
