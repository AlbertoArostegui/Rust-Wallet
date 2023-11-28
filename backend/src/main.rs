use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use backend::read_root_key_and_unseal;
use backend::AppState;
use std::{thread, time};

mod utils;
mod walgen;
mod web_methods;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    //Esperamos 0.5s para asegurar que la base de datos estará lista para aceptar conexiones
    //En caso de que no fuese suficiente porque la base de datos ha crecido mucho, habría que aumentar el tiempo que se espera y reconstruir la imagen
    //Si es la primera vez que se ejecuta el compose entero, la bbdd necesita hacer una serie de inicializaciones, por lo que seria mejor subir el tiempo a alrededor de 5s.
    let ten_millis = time::Duration::from_millis(500);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
    /*
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

    
    let (secret_key, public_key) = walgen::generate_keypair();
    let public_address = walgen::public_key_address(&public_key);
    println!("Secret key: {}", secret_key.display_secret());
    println!("Public key: {}", public_key.to_string());
    println!("Public address: {}", &public_address);
    
    
    let wallet_instance = walgen::Wallet::new(&secret_key, &public_key);
    println!("Public key: {}", wallet_instance.public_address.to_string());

    let endpoint = env::var("INFURA_RINKEBY_WS").unwrap();
    let web3_conn = walgen::establish_web3_connection(&endpoint).await;

    let block_number = web3_conn.eth().block_number().await.unwrap();
    println!("block number: {}", &block_number);

    let balance = wallet_instance.get_eth_balance(&web3_conn).await.unwrap();
    println!("Balance: {balance}");

    */
     
    let root_key: String = read_root_key_and_unseal().await; 
    let shared_state = web::Data::new(AppState { root_key: root_key.clone() });
    println!("Root key: {}", root_key);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(web_methods::check_email_exists)
            .service(web_methods::create_new_user)
            .service(web_methods::check_password)
            .service(web_methods::get_balance)
            .service(web_methods::sign_and_send)
            .app_data(shared_state.clone())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
