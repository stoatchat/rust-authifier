//! Run example with `cargo run --example rocket_mongodb`

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    use authifier::database::MongoDb;
    use authifier::Migration;
    use mongodb::{options::ClientOptions, Client};
    use utoipa::OpenApi;
    use utoipa_scalar::{Scalar, Servable};

    #[derive(OpenApi)]
    #[openapi(nest(
        (path = "/auth/account", api = rocket_authifier::routes::account::ApiDoc),
        (path = "/auth/session", api = rocket_authifier::routes::session::ApiDoc),
        (path = "/auth/mfa", api = rocket_authifier::routes::mfa::ApiDoc),
    ))]
    struct ApiDoc;

    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Valid connection URL");

    let client = Client::with_options(client_options).expect("MongoDB server");
    let database = authifier::Database::MongoDb(MongoDb(client.database("authifier")));

    for migration in [Migration::WipeAll, Migration::M2022_06_03EnsureUpToSpec] {
        database.run_migration(migration).await.unwrap();
    }

    let authifier = authifier::Authifier {
        database,
        ..Default::default()
    };

    rocket::build()
        .configure(rocket::Config { port: 8002, ..Default::default() })
        .mount("/auth/account", rocket_authifier::routes::account::routes())
        .mount("/auth/session", rocket_authifier::routes::session::routes())
        .mount("/auth/mfa", rocket_authifier::routes::mfa::routes())
        .mount("/", Scalar::with_url("/scalar", ApiDoc::openapi()))
        .manage(authifier)
}