use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use backend::models::*;
use backend::establish_connection;
use diesel::prelude::*;
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
        println!("{}", user.username);
        println!("-----------\n");
        println!("{}", user.first_name);
    }
    HttpResponse::Ok().body("eoeo")
}
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/prueba")]
pub async fn prueba(json: web::Json<Info>) -> Result<String> {
    println!("Welcome {}!", json.username);
    Ok(format!("Welcome {}!", json.username))
}