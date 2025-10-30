//! Disable a Webauthn device.
//! DELETE /mfa/webauthn
use authifier::models::{Account, ValidatedTicket};
use authifier::{Authifier, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

/// # Disable Webauthn device
///
/// Disables a Webauthn device.
#[openapi(tag = "MFA")]
#[delete("/totp?<key_name>")]
pub async fn webauthn_disable(
    authifier: &State<Authifier>,
    mut account: Account,
    _ticket: ValidatedTicket,
    key_name: &str,
) -> Result<EmptyResponse> {
    // Disable TOTP
    account.mfa.webauthn.keys.retain(|_, key| key.name != key_name);

    // Save model to database
    account.save(authifier).await.map(|_| EmptyResponse)
}