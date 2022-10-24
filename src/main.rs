// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

mod client;
use client::*;
mod model;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://api.kollider.xyz/v1/";

    let cfg = load_config()?;
    println!("config: {:?}", cfg);

    let client = KolliderClient::new(base_url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    let prices = client.get_price_ticker().await?;
    println!("prices: {:?}", prices.last_price);

    let balance = client.get_user_balances().await?;
    println!("balance in sats: {:?}", balance.cash.sat);

    let products = client.get_products().await?;
    println!("products: {:?}", products);

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
