use std::collections::HashMap;
use webauthn_rs::prelude::{AttestedPasskey, AttestedPasskeyAuthentication, AttestedPasskeyRegistration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebauthnKey {
    pub key: AttestedPasskey,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Webauthn {
    pub keys: HashMap<Vec<u8>, WebauthnKey>,
    pub pending: Option<AttestedPasskeyRegistration>,
    pub challanges: HashMap<String, AttestedPasskeyAuthentication>
}
