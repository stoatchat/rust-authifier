//! Generate a new challenge for Webauthn.
//! POST /mfa/webauthn
use authifier::models::{Account, ValidatedTicket};
use authifier::{webauthn::Webauthn, Authifier, Error, Result};
use rocket::serde::json::{
    serde_json::{to_value, Value},
    Json,
};
use rocket::State;

/// # Generate Webauthn challenge
///
/// Generate a Webauthn challenge to register a new device.
#[openapi(tag = "MFA")]
#[post("/webauthn")]
pub async fn webauthn_generate(
    authifier: &State<Authifier>,
    webauthn: &State<Webauthn>,
    mut account: Account,
    _ticket: ValidatedTicket,
) -> Result<Json<Value>> {
    // Generate a new challenge
    let challenge =
        account
            .mfa
            .webauthn
            .register_key(webauthn, &account.id, &account.email_normalised)?;

    // Save model to database
    account.save(authifier).await?;

    // Send secret to user
    to_value(challenge)
        .map_err(|_| Error::InternalError)
        .map(Json)
}
