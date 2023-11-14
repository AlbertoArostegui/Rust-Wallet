use actix_web::{get, post, web, HttpResponse, Responder, Result};
use backend::models::*;
use backend::establish_connection;
use diesel::insert_into;
use diesel::prelude::*;
use serde_derive::Serialize;
use serde::Deserialize;

use crate::utils;
use crate::walgen;

#[derive(Deserialize)]
struct Email {
    email: String,
}
#[derive(Serialize)]
struct EmailExists {
    email_exists: bool,
}

#[post("/checkEmailExists")]
pub async fn check_email_exists(json: web::Json<Email>) -> web::Json<EmailExists> {
    let connection = &mut establish_connection();

    use backend::schema::users::dsl::*;
    let results = users
        .filter(email.eq(&json.email))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    if results.len() == 0 {
        web::Json(EmailExists { email_exists: false })
    } else {
        web::Json(EmailExists { email_exists: true })
    }
}

#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
    password: String,
}

#[post("/createNewUser")]
pub async fn create_new_user(json: web::Json<NewUser>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_name = &json.name;
    let new_email = &json.email;
    let new_password = &json.password;

    use backend::schema::users::dsl::*;

    let (new_hashed_password, new_salt) = utils::hash_password(new_password);
    let (new_secret_key, new_public_key) = walgen::generate_keypair();
    let new_public_address = walgen::public_key_address(&new_public_key);

    let _ = insert_into(users)
        .values((
            name.eq(new_name),
            email.eq(new_email),
            hashed_password.eq(new_hashed_password),
            salt.eq(new_salt),
            private_key.eq(new_secret_key.display_secret().to_string()),
            public_key.eq(new_public_key.to_string()),
            address.eq(new_public_address.to_string()),
        ))
        .execute(connection);

    HttpResponse::Ok().body("Generated new user")
}

#[post("/checkPassword")]
pub async fn check_password(json: web::Json<NewUser>) -> impl Responder {
    println!("Checking if password matchers");
    let connection = &mut establish_connection();
    let new_email = &json.email;
    let new_password = &json.password;

    use backend::schema::users::dsl::*;

    let results = users
        .filter(email.eq(&new_email))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    if results.len() == 0 {
        HttpResponse::Ok().body("User does not exist")
    } else {
        let user = &results[0];
        let password_matches = utils::check_hash(&new_password, &user.salt, &user.hashed_password);
        if password_matches {
            println!("Password matches");
            HttpResponse::Ok().body("Password matches")
        } else {
            HttpResponse::Ok().body("Password does not match")
        }
    }
}