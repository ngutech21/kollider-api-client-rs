use kollider_api_client_rs::{
    client::KolliderClient,
    model::{KolliderClientConfig, OrderSide},
};

#[tokio::test]
async fn test_get_price_ticker() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = KolliderClientConfig {
        url: "https://testnet.kollider.xyz/v1".to_string(),
        api_key: "".to_string(),
        passphrase: "".to_string(),
        secret: "".to_string(),
    };
    let client = KolliderClient::new(&cfg.url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    let _prices = client.get_price_ticker().await?;
    assert_eq!("BTCUSD.PERP", _prices.symbol);
    Ok(())
}

#[tokio::test]
async fn test_get_products() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = KolliderClientConfig {
        url: "https://testnet.kollider.xyz/v1".to_string(),
        api_key: "".to_string(),
        passphrase: "".to_string(),
        secret: "".to_string(),
    };
    let client = KolliderClient::new(&cfg.url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    let products = client.get_products().await?;
    assert_eq!("1", products.btcusd_perp.unwrap().contract_size);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_place_order() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = KolliderClientConfig::load_config("config.json")?;

    // buy BTC for 1 $
    let client = KolliderClient::new(&cfg.url, &cfg.api_key, &cfg.passphrase, &cfg.secret);
    let products = client.create_order(OrderSide::Bid, 1).await?;
    dbg!(&products);
    assert_eq!(1, products.quantity);
    assert_eq!("BTCUSD.PERP", products.symbol);

    // sell BTC for 1 $
    let products = client.create_order(OrderSide::Ask, 1).await?;
    dbg!(&products);
    assert_eq!(1, products.quantity);
    assert_eq!("BTCUSD.PERP", products.symbol);
    Ok(())
}
