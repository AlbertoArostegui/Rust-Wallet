use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::models::*;
use backend::establish_connection;
use diesel::prelude::*;
use std::{thread, time};

mod keygen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    //Esperamos 0.5s para asegurar que la base de datos estará lista para aceptar conexiones
    //En caso de que no fuese suficiente porque la base de datos ha crecido mucho, habría que aumentar el tiempo que se espera y reconstruir la imagen
    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);

    let (secret_key, public_key) = keygen::generate_keypair();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/users", web::get().to(print_users))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
