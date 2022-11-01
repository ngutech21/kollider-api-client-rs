// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

mod client;
mod error;
use client::*;
mod model;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://testnet.kollider.xyz/v1";

    let cfg = load_config()?;
    println!("config: {:?}", cfg);

    let client = KolliderClient::new(base_url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    // let prices = client.get_price_ticker().await?;
    // println!("prices: {:?}", prices.last_price);

    // let balance = client.get_user_balances().await?;
    // println!("balance in sats: {:?}", balance.cash.sat);

    // let products = client.get_products().await?;
    // println!("products: {:?}", products);

    // let deposit = client.make_deposit(100).await?;
    // println!("order: {:?}", deposit.payment_request);

    let order = client.create_order().await?;
    println!("order: {:?}", order);

    // let pos = client.get_open_positions().await?;
    // println!("pos: {:?}", pos);
    //rust 1666694875POST/wallet/deposit{"amount":"100","type":"Ln"}
    //py   1666694805POST/wallet/deposit{"amount":"100","type":"Ln"}

    //{"k-signature": "nLwhx1jZeqjBrBXPo8JDKAPrWlTZzmRhWmoQX8MmMQs=",
    //"k-timestamp": "1666695351", "k-passphrase":
    //"pass987", "k-api-key": "YtUuYfPXE4r8mQhHBkAs8A=="}

    //{'k-signature': 'Y5fKo3Qm+/Ok7dlP1vQdkREdUeQBvvT5f+Gh0lZ/V/w=',
    //  'k-timestamp': '1666695172',
    //  'k-passphrase': 'pass987',
    //  'k-api-key': 'YtUuYfPXE4r8mQhHBkAs8A=='}

    Ok(())
}

fn load_config() -> Result<KolliderClientConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.json")?;
    Ok(serde_json::from_str::<KolliderClientConfig>(&content)?)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct KolliderClientConfig {
    api_key: String,
    passphrase: String,
    secret: String,
}
