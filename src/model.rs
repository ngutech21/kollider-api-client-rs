use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Products {
    #[serde(rename = "ETHUSD.PERP")]
    pub ethusd_perp: Product,
    #[serde(rename = "BTCUSD.PERP")]
    pub btcusd_perp: Product,
    #[serde(rename = "BTCEUR.PERP")]
    pub btceur_perp: Product,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub symbol: String,
    pub contract_size: String,
    pub max_leverage: String,
    pub base_margin: String,
    pub maintenance_margin: String,
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
