//! Initiates a new MFA ticket with a webauthn challenge.
//! POST /mfa/ticket/webauthn
use authifier::models::{Account, MFATicket};
use authifier::{Authifier, Error, Result};
use rocket::serde::json::Json;
use rocket::State;

/// # Initiates Webauthn MFA ticket
///
/// Initiates a new MFA ticket with a webauthn challenge.
#[openapi(tag = "MFA")]
#[post("/ticket/webauthn")]
pub async fn initiate_webauthn_ticket(
    authifier: &State<Authifier>,
    mut account: Account,
) -> Result<Json<MFATicket>> {
    if !account.mfa.webauthn.is_enabled() {
        return Err(Error::DisallowedMFAMethod);
    };

    // Create a new unvalidated ticket for this account
    let mut ticket = MFATicket::new(account.id.clone(), false);
    ticket.save(authifier).await?;

    ticket.webauthn_challenge = Some(
        account.mfa.webauthn.generate_auth_challenge(
            ticket.id.clone(),
            authifier
                .webauthn
                .as_ref()
                .ok_or(Error::DisallowedMFAMethod)?,
        )?,
    );
    account.save(authifier).await?;

    Ok(Json(ticket))
}
