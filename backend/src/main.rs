use bdk::{SyncOptions, descriptor};
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::{Wallet, database::MemoryDatabase};
use bdk::bitcoin::Network;
use dotenv::{self, from_filename, dotenv};
use std::env::var;

fn main() -> anyhow::Result<()> {
    from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = var("WALLET_DESCRIPTOR").unwrap();
    dbg!(&descriptor);

    let wallet = Wallet::new(&descriptor, None, Network::Testnet, MemoryDatabase::default())?;
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    wallet.sync(&blockchain, SyncOptions::default());
    let balance = wallet.get_balance();
    let address = wallet.get_address(bdk::wallet::AddressIndex::New);

    dbg!(balance);
    dbg!(address);
}

fn setup() -> String {
    from_filename(".env");
    dotenv();

    let descriptor = var("WALLET_DESCRIPTOR");

    match descriptor {
        Ok(descriptor) => descriptor,
        Err(_) => "Couldn't fetch WALLET_DESCRIPTOR env variable".to_string()
    }
}