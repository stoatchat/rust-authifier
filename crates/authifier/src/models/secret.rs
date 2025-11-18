use rand::distributions::{Alphanumeric, DistString};

/// Secret model
#[derive(Serialize, Deserialize, Clone)]
pub struct Secret {
    #[serde(rename = "_id")]
    pub id: String,
    secret: String,
}

impl Secret {
    pub fn new() -> Self {
        Self {
            id: "0".to_string(),
            secret: Alphanumeric.sample_string(&mut rand::thread_rng(), 512),
        }
    }

    pub fn expose(&self) -> &str {
        &self.secret
    }
}

impl Default for Secret {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secret: String = std::iter::repeat_n('X', self.secret.len()).collect();

        f.debug_tuple("Secret").field(&secret).finish()
    }
}
