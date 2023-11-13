use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use backend::models::*;
use backend::establish_connection;
use diesel::prelude::*;
use serde_derive::Serialize;
use std::env;
use serde::Deserialize;
use std::{thread, time};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/userExists/:user")]
async fn print_users() -> impl Responder {

    let connection = &mut establish_connection();

    use backend::schema::users::dsl::*;
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("-----------\n");
        println!("{}", user.email);
    }
    HttpResponse::Ok().body("eoeo")
}
#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

#[post("/prueba")]
pub async fn prueba(json: web::Json<Info>) -> Result<String> {
    let response = format!("Welcome {}! with email {}", json.name, json.email);
    println!("{}", response);
    Ok(response)
}
#[derive(Deserialize)]
struct Email {
    email: String,
}
#[derive(Serialize)]
struct EmailExists {
    email_exists: bool,
}

#[post("/checkEmailExists")]
pub async fn checkEmailExists(json: web::Json<Email>) -> web::Json<EmailExists> {
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