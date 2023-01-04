use std::fmt::Display;
use std::fs;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Products {
    #[serde(rename = "ETHUSD.PERP")]
    pub ethusd_perp: Option<Product>,
    #[serde(rename = "BTCUSD.PERP")]
    pub btcusd_perp: Option<Product>,
    #[serde(rename = "BTCEUR.PERP")]
    pub btceur_perp: Option<Product>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub symbol: String,
    pub contract_size: String,
    pub max_leverage: String,
    pub base_margin: String,
    pub is_inverse_priced: bool,
    pub price_dp: i64,
    pub underlying_symbol: String,
    pub last_price: String,
    pub tick_size: String,
    pub risk_limit: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PriceTicker {
    pub best_ask: String,
    pub best_bid: String,
    pub last_price: String,
    pub last_quantity: i64,
    pub last_side: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserBalances {
    pub cash: Cash,
    pub cross_margin: String,
    pub isolated_margin: IsolatedMargin,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Cash {
    pub kkp: String,
    pub sat: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: i64,
    pub order_type: String,
    pub side: String,
    pub quantity: i64,
    pub symbol: String,
    pub leverage: i64,
    pub margin_type: String,
    pub settlement_type: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IsolatedMargin {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpenPositions {
    #[serde(rename = "BTCUSD.PERP")]
    pub btcusd_perp: BtcusdPerp,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BtcusdPerp {
    pub uid: i64,
    pub timestamp: i64,
    pub symbol: String,
    pub upnl: String,
    pub leverage: String,
    pub entry_price: String,
    pub side: String,
    pub quantity: String,
    pub liq_price: String,
    pub open_order_ids: Vec<i32>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub payment_request: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResult {
    pub timestamp: i64,
    pub order_id: i64,
    pub ext_order_id: String,
    pub uid: i64,
    pub symbol: String,
    pub quantity: i64,
    pub order_type: String,
    pub price: i64,
    pub leverage: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrders {
    #[serde(rename = "BTCUSD.PERP")]
    pub btcusd_perp: Vec<OpenOrderBtcusdPerp>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrderBtcusdPerp {
    pub order_id: i64,
    pub uid: i64,
    pub price: i64,
    pub timestamp: i64,
    pub filled: i64,
    pub ext_order_id: String,
    pub order_type: String,
    pub advanced_order_type: Value,
    pub trigger_price_type: Value,
    pub side: String,
    pub quantity: i64,
    pub symbol: String,
    pub leverage: i64,
    pub margin_type: String,
    pub settlement_type: String,
    pub origin: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub created_at: CreatedAt,
    pub email: String,
    pub lnauth_enabled: bool,
    pub user_type: String,
    pub username: String,
    pub validated_email: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreatedAt {
    pub nanos_since_epoch: i64,
    pub secs_since_epoch: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMarginResult {
    pub ext_id: String,
    pub uid: i64,
    pub symbol: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResult {
    pub timestamp: i64,
    pub reason: String,
    pub order_id: i64,
    pub symbol: String,
    pub order_type: String,
    pub uid: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KolliderClientConfig {
    pub url: String,
    pub api_key: String,
    pub passphrase: String,
    pub secret: String,
}

pub enum OrderSide {
    Bid,
    Ask,
}

impl Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Ask => f.write_str("Ask"),
            OrderSide::Bid => f.write_str("Bid"),
        }
    }
}

impl KolliderClientConfig {
    pub fn load_config(
        config_name: &str,
    ) -> Result<KolliderClientConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(config_name)?;
        Ok(serde_json::from_str::<KolliderClientConfig>(&content)?)
    }
}
