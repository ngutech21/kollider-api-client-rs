use data_encoding::BASE64;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

use chrono::Utc;
use ring::hmac;

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

    // FIXME wrap result
    fn create_headers(&self, path: &str) -> HeaderMap {
        let timestamp: i64 = Utc::now().timestamp();
        let sig = Self::generate_signature(timestamp, self.secret, path, "GET");

        let mut header = HeaderMap::new();
        header.append("k-signature", HeaderValue::from_str(sig.as_str()).unwrap());

        header.append(
            "k-timestamp",
            HeaderValue::from_str(&format!("{}", timestamp)).unwrap(),
        );
        header.append(
            "k-passphrase",
            HeaderValue::from_str(self.passphrase).unwrap(),
        );
        header.append("k-api-key", HeaderValue::from_str(self.api_key).unwrap());
        header
    }

    fn generate_signature(timestamp: i64, secretb64: &str, path: &str, method: &str) -> String {
        let pre_hash = format!("{}{}{}", timestamp, method, path);
        let res = BASE64.decode(secretb64.as_bytes()).unwrap();
        let key = hmac::Key::new(hmac::HMAC_SHA256, &res);
        let signature = hmac::sign(&key, pre_hash.as_bytes());
        let sig_encoded = BASE64.encode(signature.as_ref());
        println!("sig: {:?}", sig_encoded);
        sig_encoded
    }

    pub async fn get_price_ticker(&self) -> Result<PriceTicker, reqwest::Error> {
        let path = "market/ticker?symbol=BTCUSD.PERP";
        let req = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await?;

        req.json::<PriceTicker>().await
    }

    pub async fn get_products(&self) -> Result<Products, reqwest::Error> {
        let res = self
            .client
            .get("https://api.kollider.xyz/v1/market/products")
            .send()
            .await?;

        res.json::<Products>().await
    }

    pub async fn get_user_balances(&self) -> Result<UserBalances, reqwest::Error> {
        let path = "/user/balances";
        let res = self
            .client
            .get("https://api.kollider.xyz/v1/user/balances")
            .headers(Self::create_headers(self, path))
            .send()
            .await?;

        res.json::<UserBalances>().await
        //let result = res.unwrap().text().await;
    }
}
