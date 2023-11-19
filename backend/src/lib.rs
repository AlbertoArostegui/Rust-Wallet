use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Couldn't find database url, set DATABASE_URL in the .env file");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_vault_connection() -> VaultClient {
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://vault:8200")
            .token("token")
            .build()
            .unwrap()
    ).unwrap();

    client
}