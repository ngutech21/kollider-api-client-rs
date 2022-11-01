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
    let cfg = load_config()?;
    let client = KolliderClient::new(&cfg.url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    // let prices = client.get_price_ticker().await?;
    // println!("prices: {:?}", prices.last_price);

    // let balance = client.get_user_balances().await?;
    // println!("balance in sats: {:?}", balance.cash.sat);

    // let products = client.get_products().await?;
    // println!("products: {:?}", products);

    // let deposit = client.make_deposit(100).await?;
    // println!("order: {:?}", deposit.payment_request);

    // let order = client.create_order(1).await?;
    // println!("order id: {} ext: {}", order.order_id, order.ext_order_id);

    // let pos = client.get_open_positions().await?;
    // println!("pos: {:?}", pos);

    // let order = client.get_open_orders().await?;
    // println!("pos: {:?}", order);

    // let request = "lntXXX";
    // let withdrawal = client.make_withdrawal(1000, request).await?;
    // println!("order: {:?}", withdrawal);

    let account = client.get_user_account().await?;
    println!("order: {:?}", account);

    Ok(())
}

fn load_config() -> Result<KolliderClientConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.json")?;
    Ok(serde_json::from_str::<KolliderClientConfig>(&content)?)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct KolliderClientConfig {
    url: String,
    api_key: String,
    passphrase: String,
    secret: String,
}
