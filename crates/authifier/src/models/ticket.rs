use webauthn_rs::prelude::RequestChallengeResponse;

/// Multi-factor auth ticket
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemas", derive(JsonSchema))]
pub struct MFATicket {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,

    /// Account Id
    pub account_id: String,

    /// Unique Token
    pub token: String,

    /// Whether this ticket has been validated
    /// (can be used for account actions)
    pub validated: bool,

    /// Whether this ticket is authorised
    /// (can be used to log a user in)
    pub authorised: bool,

    /// TOTP code at time of ticket creation
    pub last_totp_code: Option<String>,

    /// Webauthn challenge
    #[schemars(with = "serde_json::Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webauthn_challenge: Option<RequestChallengeResponse>,
}

impl PartialEq for MFATicket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.account_id == other.account_id
            && self.token == other.token
            && self.validated == other.validated
            && self.authorised == other.authorised
            && self.last_totp_code == other.last_totp_code
            // skip webauthn_challenge
    }
}

/// Ticket which is guaranteed to be valid for use
///
/// If used in a Rocket guard, it will be consumed on match
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedTicket(pub MFATicket);

/// Ticket which is guaranteed to not be valid for use
#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedTicket(pub MFATicket);
