use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use sha2::{Sha256, Digest};
use web3::types::U256;

pub fn get_time_ns() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

pub fn eth_to_wei(eth_val: f64) -> U256 {
    let result = eth_val * 1_000_000_000_000_000_000.0;
    let result = result as u128;
    U256::from(result)
}

//Create a function that takes a password and returns a salted hash and the salt

pub fn hash_password(password: &str) -> (String, String) {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(&salt);
    let result = hasher.finalize();
    let result = hex::encode(result);
    let salt = hex::encode(salt);
    (result, salt)
}

pub fn check_hash(password: &str, salt: &str, hash: &str) -> bool {
    let salt = hex::decode(salt).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(&salt);
    let result = hasher.finalize();
    let result = hex::encode(result);
    result == hash
}