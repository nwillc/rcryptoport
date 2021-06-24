use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use ureq;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerInfo {
    pub currency: String,
    pub price: Decimal,
    pub price_timestamp: String,
}

pub fn prices(app_id: &String, currencies: &Vec<String>) -> Result<HashMap<String,Decimal>, String> {
    let ids = currencies.join(",");
    return match ureq::get("https://api.nomics.com/v1/currencies/ticker")
        .query("key", app_id.as_str())
        .query("ids", ids.as_str())
        .query("interval", "1d")// add ?foo=bar+baz
        .call() {
        Err(err) => Err(err.to_string()),
        Ok(response) => {
            match response.into_string() {
                Err(err) => Err(err.to_string()),
                Ok(body) => {
                    let tis: Result<Vec<TickerInfo>, Error> = serde_json::from_str(&body);
                    match tis {
                        Err(err) => Err(err.to_string()),
                        Ok(payload) => {
                            let map: HashMap<String,Decimal> = payload.iter().map(|ticker_info| (ticker_info.currency.clone(), ticker_info.price.clone())).collect();
                            Ok(map)
                        },
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_prices() {
        let app_id = env::var("NOMICS_APP_ID").unwrap();
        let currencies = vec!["BTC".to_string(), "ETH".to_string()];
        match super::prices(&app_id, &currencies) {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(prices) => {
                for currency in currencies {
                  assert!(prices.contains_key(&currency), "missing currency {}", currency)
                }
            }
        }
    }
}
