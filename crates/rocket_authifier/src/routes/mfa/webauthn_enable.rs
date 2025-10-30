//! Enables a Webauthn device.
//! PUT /mfa/webauthn
use authifier::models::{Account};
use authifier::{webauthn::Webauthn, Authifier, Result, Error};
use rocket::serde::json::{Json, Value, from_value};
use rocket::State;
use rocket_empty::EmptyResponse;

/// # Enable Webauthn
///
/// Enables a Webauthn device.
#[openapi(tag = "MFA")]
#[put("/webauthn?<key_name>", data = "<data>")]
pub async fn webauthn_enable(
    authifier: &State<Authifier>,
    webauthn: &State<Webauthn>,
    mut account: Account,
    key_name: String,
    data: Json<Value>,
) -> Result<EmptyResponse> {
    let cred = from_value(data.into_inner()).map_err(|_| Error::OperationFailed)?;

    // Enable Webauthn key
    account.mfa.webauthn.finish_register_key(webauthn, key_name, &cred)?;

    // Save model to database
    account.save(authifier).await.map(|_| EmptyResponse)
}
