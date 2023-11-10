use std::time::{SystemTime, UNIX_EPOCH};
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