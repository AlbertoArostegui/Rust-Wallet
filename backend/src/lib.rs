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