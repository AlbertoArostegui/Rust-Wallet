use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde_json::json;
use std::{env, io::{self, BufRead}, path::Path, fs::File};
use reqwest::Error;

pub mod models;
pub mod schema;

pub struct AppState {
    pub root_key: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Couldn't find database url, set DATABASE_URL in the .env file");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn insert_into_vault(token: &str, key: &str, val: &str) {
    let url = format! ("http://secret-vault:8200/v1/secret/api/secretkeys/{}", key);
    
    let data = json!({
        "data": {
            "privatekey": val
        }
    });
    let client = reqwest::Client::new();
    let res = client.put(url)
        .header("Content-Type", "application/json")
        .header("X-Vault-Token", token)
        .json(&data)
        .send()
        .await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Successfully inserted key-value pair into Vault");
            } else {
                println!("Error inserting key-value pair into Vault: {}", response.status());
            }
        },
        Err(e) => {
            println!("Error inserting key-value pair into Vault: {}", e);
        }
    }
}

pub async fn retrieve_from_vault(token: &str, key: &str) -> String {
    let url = format!("http://secret-vault:8200/v1/secret/api/secretkeys/{}", key);
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header("X-Vault-Token", token)
        .send()
        .await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_data: serde_json::Value = response.json().await.unwrap();
                if let Some(value) = json_data["data"]["privatekey"].as_str() {
                    return value.to_string();
                } else {
                    println!("Key not found in Vault");
                }
            } else {
                println!("Error retrieving value from Vault: {}", response.status());
            }
        },
        Err(e) => {
            println!("Error retrieving value from Vault: {}", e);
        }
    }
    String::new()
}

pub async fn read_root_key_and_unseal() -> String {
    let path = Path::new("vault/keys.txt");
    let search_string = "Initial Root Token:";
    let mut result_key = String::from("Placeholder?");
    let file = File::open(&path);
    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                if line.contains(search_string) {
                    let splitted: Vec<&str> = line.split(":").collect(); 
                    result_key = splitted[1].trim().to_string();   
                } else if line.contains("Unseal Key 1:") {
                    let splitted: Vec<&str> = line.split(":").collect(); 
                    let unseal_key = splitted[1].trim().to_string();   
                    let res = unseal_vault(unseal_key).await;
                    match res {
                        Ok(_) => {
                            println!("Unsealed vault");
                        },
                        Err(e) => {
                            println!("Error unsealing vault: {}", e);
                        }
                    }
                }                 
            }
        },
        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }
    result_key
}

async fn unseal_vault(key: String) -> Result<(), Error> {
    let url = "http://secret-vault:8200/v1/sys/unseal";
    let data = json!({
        "key": key
    });
    let client = reqwest::Client::new();
    let res = client.put(url)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Vault unsealed");
            } else {
                println!("Error unsealing vault: {}", response.status());
            }
        },
        Err(e) => {
            println!("Error unsealing vault: {}", e);
        }
    }
    Ok(())
}