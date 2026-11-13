use std::collections::HashMap;
use webauthn_rs::prelude::{CredentialID, SecurityKey, SecurityKeyAuthentication, SecurityKeyRegistration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebauthnKey {
    pub key: SecurityKey,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Webauthn {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub keys: HashMap<CredentialID, WebauthnKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<SecurityKeyRegistration>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub challanges: HashMap<String, SecurityKeyAuthentication>
}
