use std::collections::HashMap;

use data_encoding::BASE64;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use chrono::Utc;
use ring::hmac;

use crate::error::KolliderClientError;

use super::model::*;

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
        path: &str,
        timestamp: i64,
        signature: &str,
    ) -> Result<HeaderMap, KolliderClientError> {
        // let timestamp: i64 = Utc::now().timestamp();
        // let sig = Self::generate_signature(timestamp, self.secret, path, "GET")?;

        let mut header = HeaderMap::new();
        header.append("k-signature", HeaderValue::from_str(signature)?);

        header.append(
            "k-timestamp",
            HeaderValue::from_str(&format!("{}", timestamp))?, // FIXME
        );
        header.append("k-passphrase", HeaderValue::from_str(self.passphrase)?);
        header.append("k-api-key", HeaderValue::from_str(self.api_key)?);
        Ok(header)
    }

    fn create_get_headers(&self, path: &str) -> Result<HeaderMap, KolliderClientError> {
        let timestamp: i64 = Utc::now().timestamp();
        let pre_hash = format!("{}{}{}", timestamp, "GET", path);
        let sig = Self::generate_signature(self.secret, &pre_hash)?;
        Self::create_headers(self, path, timestamp, &sig)
    }

    fn create_post_headers(
        &self,
        path: &str,
        body: &str,
    ) -> Result<HeaderMap, KolliderClientError> {
        let timestamp: i64 = Utc::now().timestamp();
        let pre_hash = format!("{}{}{}{}", timestamp, "POST", path, body);
        let sig = Self::generate_signature(self.secret, &pre_hash)?;
        Self::create_headers(self, path, timestamp, &sig)
    }

    fn generate_signature(secretb64: &str, pre_hash: &str) -> Result<String, KolliderClientError> {
        let res = BASE64.decode(secretb64.as_bytes())?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, &res);
        let signature = hmac::sign(&key, pre_hash.as_bytes());
        let sig_encoded = BASE64.encode(signature.as_ref());
        println!("sig: {:?}", sig_encoded);
        Ok(sig_encoded)
    }

    // fn generate_signature(
    //     timestamp: i64,
    //     secretb64: &str,
    //     path: &str,
    //     method: &str,
    // ) -> Result<String, KolliderClientError> {
    //     let pre_hash = format!("{}{}{}", timestamp, method, path);
    //     let res = BASE64.decode(secretb64.as_bytes())?;
    //     let key = hmac::Key::new(hmac::HMAC_SHA256, &res);
    //     let signature = hmac::sign(&key, pre_hash.as_bytes());
    //     let sig_encoded = BASE64.encode(signature.as_ref());
    //     println!("sig: {:?}", sig_encoded);
    //     Ok(sig_encoded)
    // }

    pub async fn get_price_ticker(&self) -> Result<PriceTicker, KolliderClientError> {
        let path = "market/ticker?symbol=BTCUSD.PERP";
        let req = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        Ok(req.json::<PriceTicker>().await.unwrap())
    }

    pub async fn get_products(&self) -> Result<Products, KolliderClientError> {
        let path = "market/products";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        Ok(res.json::<Products>().await.unwrap())
    }

    pub async fn get_user_balances(&self) -> Result<UserBalances, KolliderClientError> {
        let path = "/user/balances";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;

        Ok(res.json::<UserBalances>().await.unwrap())
    }

    pub async fn get_open_orders(&self) -> Result<String, KolliderClientError> {
        let path = "/orders/open";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;

        //Ok(res.json::<UserBalances>().await.unwrap())
        Ok(res.text().await.unwrap())
    }

    pub async fn get_open_positions(&self) -> Result<OpenPositions, KolliderClientError> {
        let path = "/positions";
        let res = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .headers(Self::create_get_headers(self, path)?)
            .send()
            .await?;

        Ok(res.json::<OpenPositions>().await.unwrap())
        //Ok(res.text().await.unwrap())
    }

    pub async fn create_order(&self) -> Result<String, KolliderClientError> {
        let path = "/orders";

        let mut body: HashMap<String, String> = HashMap::new();
        body.insert("price".to_string(), "19158.0".to_string());
        body.insert("order_type".to_string(), "Limit".to_string());
        body.insert("side".to_string(), "Bid".to_string());
        body.insert("quantity".to_string(), "1".to_string());
        body.insert("symbol".to_string(), "BTCUSD.PERP".to_string());
        body.insert("leverage".to_string(), "2".to_string());
        body.insert("margin_type".to_string(), "Isolated".to_string());
        body.insert("settlement_type".to_string(), "Delayed".to_string());
        let request_body = serde_json::to_string(&body).unwrap(); // FIXME

        let res = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(Self::create_post_headers(self, path, &request_body)?)
            .body(request_body)
            .send()
            .await?;
        let st = res.status();
        //res.error_for_status();

        let result = res.text().await.unwrap();
        println!("order result {:?} {:?}", result, st.to_string());
        Ok(result)
    }
}
