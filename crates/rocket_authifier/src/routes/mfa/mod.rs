use rocket::Route;
use crate::SecurityAddon;

pub mod create_ticket;
pub mod fetch_recovery;
pub mod fetch_status;
pub mod generate_recovery;
pub mod get_mfa_methods;
pub mod totp_disable;
pub mod totp_enable;
pub mod totp_generate_secret;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_ticket::create_ticket,
        fetch_status::fetch_status,
        fetch_recovery::fetch_recovery,
        generate_recovery::generate_recovery,
        get_mfa_methods::get_mfa_methods,
        totp_disable::totp_disable,
        totp_enable::totp_enable,
        totp_generate_secret::totp_generate_secret,
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![
        create_ticket::create_ticket,
        fetch_status::fetch_status,
        fetch_recovery::fetch_recovery,
        generate_recovery::generate_recovery,
        get_mfa_methods::get_mfa_methods,
        totp_disable::totp_disable,
        totp_enable::totp_enable,
        totp_generate_secret::totp_generate_secret,
    ]
}
