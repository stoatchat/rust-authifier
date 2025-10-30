use std::ops::Deref;

use crate::{
    models::webauthn::{Webauthn, WebauthnKey},
    Error, Result,
};
use ulid::Ulid;
use webauthn_rs::{
    prelude::{
        AttestationCaList, AttestationCaListBuilder, AuthenticatorAttachment,
        CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential,
        RequestChallengeResponse, Uuid,
    },
    Webauthn as WebauthnCore,
};

impl Webauthn {
    pub fn is_enabled(&self) -> bool {
        !self.keys.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn register_key(
        &mut self,
        webauthn: &WebauthnCore,
        account_id: &str,
        account_name: &str,
    ) -> Result<CreationChallengeResponse> {
        // TODO: limit key count

        let ulid = Ulid::from_string(account_id).unwrap();

        // TODO: fill this list somehow (???)
        let attestation_ca_list: AttestationCaList = AttestationCaListBuilder::new().build();

        let (challenge, reg) = webauthn
            .start_attested_passkey_registration(
                Uuid::from_u128(ulid.0),
                account_name,
                account_name,
                Some(
                    self.keys
                        .values()
                        .map(|key| key.key.cred_id().clone())
                        .collect(),
                ),
                attestation_ca_list,
                Some(AuthenticatorAttachment::CrossPlatform),
            )
            .map_err(|_| Error::OperationFailed)?;

        self.pending = Some(reg);

        Ok(challenge)
    }

    pub fn finish_register_key(
        &mut self,
        webauthn: &WebauthnCore,
        key_name: String,
        cred: &RegisterPublicKeyCredential,
    ) -> Result<()> {
        if self.keys.values().find(|v| &v.name == &key_name).is_some() {
            return Err(Error::KeyAlreadyExists)
        };

        if let Some(reg) = self.pending.take() {
            let passkey = webauthn
                .finish_attested_passkey_registration(cred, &reg)
                .map_err(|_| Error::OperationFailed)?;

            self.keys.insert(
                passkey.cred_id().deref().clone(),
                WebauthnKey {
                    key: passkey,
                    name: key_name,
                },
            )
        } else {
            return Err(Error::OperationFailed);
        };

        Ok(())
    }

    pub fn generate_auth_challenge(
        &mut self,
        ticket: String,
        webauthn: &WebauthnCore,
    ) -> Result<RequestChallengeResponse> {
        let keys = self
            .keys
            .values()
            .map(|key| key.key.clone())
            .collect::<Vec<_>>();

        let (challenge, auth) = webauthn
            .start_attested_passkey_authentication(&keys)
            .map_err(|_| Error::OperationFailed)?;

        self.challanges.insert(ticket, auth);

        Ok(challenge)
    }

    pub fn validate_auth(
        &mut self,
        ticket: &str,
        webauthn: &WebauthnCore,
        cred: &PublicKeyCredential,
    ) -> Result<()> {
        let auth = self.challanges.get(ticket).ok_or(Error::OperationFailed)?;

        let res = webauthn
            .finish_attested_passkey_authentication(cred, auth)
            .map_err(|_| Error::OperationFailed)?;

        self.challanges.remove(ticket);

        if res.needs_update() {
            self.keys
                .get_mut(res.cred_id().deref())
                .unwrap()
                .key
                .update_credential(&res);
        };

        Ok(())
    }
}
