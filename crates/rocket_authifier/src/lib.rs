use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate utoipa;

// Rust compiler seems to think
// this isn't used even though
// it is used.
#[cfg(test)]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

pub mod routes;

#[cfg(test)]
pub mod test;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_default();

        components.add_security_scheme(
            "Session-Token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(
                "X-Session-Token".to_string(),
            ))),
        );

        components.add_security_scheme(
            "MFA-Ticket",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-MFA-Ticket".to_string()))),
        );
    }
}

#[test]
fn test() {}
