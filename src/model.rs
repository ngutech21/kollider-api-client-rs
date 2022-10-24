use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    #[serde(rename = "ETHUSD.PERP")]
    pub ethusd_perp: Product,
    #[serde(rename = "BTCUSD.PERP")]
    pub btcusd_perp: Product,
    #[serde(rename = "BTCEUR.PERP")]
    pub btceur_perp: Product,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub symbol: String,
    #[serde(rename = "contract_size")]
    pub contract_size: String,
    #[serde(rename = "max_leverage")]
    pub max_leverage: String,
    #[serde(rename = "base_margin")]
    pub base_margin: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "is_inverse_priced")]
    pub is_inverse_priced: bool,
    #[serde(rename = "price_dp")]
    pub price_dp: i64,
    #[serde(rename = "underlying_symbol")]
    pub underlying_symbol: String,
    #[serde(rename = "last_price")]
    pub last_price: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(rename = "risk_limit")]
    pub risk_limit: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTicker {
    #[serde(rename = "best_ask")]
    pub best_ask: String,
    #[serde(rename = "best_bid")]
    pub best_bid: String,
    #[serde(rename = "last_price")]
    pub last_price: String,
    #[serde(rename = "last_quantity")]
    pub last_quantity: i64,
    #[serde(rename = "last_side")]
    pub last_side: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBalances {
    pub cash: Cash,
    #[serde(rename = "cross_margin")]
    pub cross_margin: String,
    #[serde(rename = "isolated_margin")]
    pub isolated_margin: IsolatedMargin,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cash {
    #[serde(rename = "KKP")]
    pub kkp: String,
    #[serde(rename = "SAT")]
    pub sat: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMargin {}
