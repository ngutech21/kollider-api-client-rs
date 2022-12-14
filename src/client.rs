#![allow(dead_code)]
use super::model::*;
use crate::error::KolliderClientError;
use chrono::{SecondsFormat, Utc};
use data_encoding::BASE64;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use ring::hmac;

#[derive(Clone)]
pub struct KolliderClient<'a> {
    client: Client,
    api_key: &'a str,
    passphrase: &'a str,
    secret: &'a str,
    base_url: &'a str,
}

impl<'a> KolliderClient<'a> {
    pub fn new(
        base_url_param: &'a str,
        apikey_param: &'a str,
        passphrase_param: &'a str,
        secret_param: &'a str,
    ) -> KolliderClient<'a> {
        KolliderClient {
            client: reqwest::Client::new(),
            base_url: base_url_param,
            api_key: apikey_param,
            passphrase: passphrase_param,
            secret: secret_param,
        }
    }

    fn create_headers(
        &self,
        timestamp: &str,
        signature: &str,
    ) -> Result<HeaderMap, KolliderClientError> {
        let mut header = HeaderMap::new();
        header.append(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        header.append("k-signature", HeaderValue::from_str(signature)?);

        header.append("k-timestamp", HeaderValue::from_str(timestamp)?);
        header.append("k-passphrase", HeaderValue::from_str(self.passphrase)?);
        header.append("k-api-key", HeaderValue::from_str(self.api_key)?);
        Ok(header)
    }

    fn create_get_headers(&self, path: &str) -> Result<HeaderMap, KolliderClientError> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let pre_hash = format!("{}{}{}", timestamp, "GET", path);
        let sig = Self::generate_signature(self.secret, &pre_hash)?;
        Self::create_headers(self, &timestamp, &sig)
    }

    fn create_post_headers(
        &self,
        path: &str,
        body: &str,
    ) -> Result<HeaderMap, KolliderClientError> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let pre_hash = format!("{}{}{}{}", timestamp, "POST", path, body);
        let sig = Self::generate_signature(self.secret, &pre_hash)?;
        Self::create_headers(self, &timestamp, &sig)
    }

    fn create_delete_headers(
        &self,
        path: &str,
        auth_body: &str,
    ) -> Result<HeaderMap, KolliderClientError> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let pre_hash = format!("{}{}{}{}", timestamp, "DELETE", path, auth_body);
        let sig = Self::generate_signature(self.secret, &pre_hash)?;
        Self::create_headers(self, &timestamp, &sig)
    }

    fn generate_signature(secretb64: &str, pre_hash: &str) -> Result<String, KolliderClientError> {
        let res = BASE64.decode(secretb64.as_bytes())?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, &res);
        let signature = hmac::sign(&key, pre_hash.as_bytes());
        let sig_encoded = BASE64.encode(signature.as_ref());
        Ok(sig_encoded)
    }

    pub async fn get_price_ticker(&self) -> Result<PriceTicker, KolliderClientError> {
        let path = "/market/ticker?symbol=BTCUSD.PERP";
        let req = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        Ok(req.json::<PriceTicker>().await?)
    }

    pub async fn get_products(&self) -> Result<Products, KolliderClientError> {
        let path = "/market/products";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        Ok(res.json::<Products>().await?)
    }

    pub async fn get_user_balances(&self) -> Result<UserBalances, KolliderClientError> {
        let path = "/user/balances";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;
        Ok(res.json::<UserBalances>().await?)
    }

    pub async fn get_user_account(&self) -> Result<UserAccount, KolliderClientError> {
        let path = "/user/account";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;
        Ok(res.json::<UserAccount>().await?)
    }

    pub async fn get_open_orders(&self) -> Result<OpenOrders, KolliderClientError> {
        let path = "/orders/open";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;
        Ok(res.json::<OpenOrders>().await?)
    }

    pub async fn get_orders(&self) -> Result<String, KolliderClientError> {
        let path = "/orders";

        let res = self
            .client
            .get(format!(
                "{}{}?symbol=BTCUSD.PERP&limit=3",
                self.base_url, path
            ))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;

        let result = res.text().await?;
        Ok(result)
    }

    pub async fn get_open_positions(&self) -> Result<OpenPositions, KolliderClientError> {
        let path = "/positions";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;
        Ok(res.json::<OpenPositions>().await?)
    }

    pub async fn create_order(
        &self,
        order_side: OrderSide,
        amount_usd: i8,
    ) -> Result<CreateOrderResult, KolliderClientError> {
        let path = "/orders";

        let request_body = serde_json::json!({
            "price": 20595,
            "order_type": "Market",
            "side": order_side.to_string(),
            "quantity": amount_usd,
            "symbol": "BTCUSD.PERP",
            "leverage": 100,
            "margin_type": "Isolated",
            "settlement_type": "Delayed"
        })
        .to_string();

        let res = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(Self::create_post_headers(self, path, &request_body)?)
            .body(request_body)
            .send()
            .await?;

        let result = res.json::<CreateOrderResult>().await?;
        Ok(result)
    }

    pub async fn cancel_order(
        &self,
        order_id: i32,
    ) -> Result<CancelOrderResult, KolliderClientError> {
        let path = "/orders";

        let auth_body = serde_json::json!({
            "order_id": order_id,
            "symbol": "BTCUSD.PERP",
        })
        .to_string();

        println!("path={}", path);
        let res = self
            .client
            .delete(format!(
                "{}{}?symbol=BTCUSD.PERP&order_id={}",
                self.base_url, path, order_id
            ))
            .headers(Self::create_delete_headers(self, path, &auth_body)?)
            .send()
            .await?;

        Ok(res.json::<CancelOrderResult>().await?)
    }

    pub async fn make_deposit(&self, sats: i32) -> Result<PaymentRequest, KolliderClientError> {
        let path = "/wallet/deposit";

        let request_body = serde_json::json!({
            "type": "Ln",
            "amount": sats,
        })
        .to_string();

        let res = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(Self::create_post_headers(self, path, &request_body)?)
            .body(request_body)
            .send()
            .await?;
        Ok(res.json::<PaymentRequest>().await?)
    }

    pub async fn make_withdrawal(
        &self,
        amount: i32,
        payment_request: &str,
    ) -> Result<String, KolliderClientError> {
        let path = "/wallet/withdrawal";

        let request_body = serde_json::json!({
            "type": "Ln",
            "payment_request": payment_request,
            "amount": amount,
        })
        .to_string();

        let res = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(Self::create_post_headers(self, path, &request_body)?)
            .body(request_body)
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn change_margin(
        &self,
        action: &str,
        amount_sats: i32,
    ) -> Result<ChangeMarginResult, KolliderClientError> {
        let path = "/change_margin";

        // action: Add Delete
        // amount in sats

        let request_body = serde_json::json!({
            "symbol": "BTCUSD.PERP",
            "action": action,
            "amount": amount_sats,
        })
        .to_string();

        let res = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(Self::create_post_headers(self, path, &request_body)?)
            .body(request_body)
            .send()
            .await?;
        Ok(res.json::<ChangeMarginResult>().await?)
    }
}
