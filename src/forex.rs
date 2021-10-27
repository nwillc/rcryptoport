use rust_decimal::prelude::*;
use serde::Deserialize;
use serde_json::Error;

#[derive(Deserialize, Debug)]
struct FXPrice {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "c")]
    price: Decimal,
}

#[derive(Deserialize, Debug)]
struct LatestResponse {
    status: bool,
    code: u8,
    msg: String,
    response: Vec<FXPrice>,
}

pub fn get_fx_rate(
    app_id: &str,
    base: &str,
    quoted: &str,
) -> Result<Decimal, String> {
    let symbol = format!("{}/{}", base, quoted);
    return match ureq::get("https://fcsapi.com/api-v3/forex/latest")
        .query("access_key", app_id)
        .query("symbol", symbol.as_str())
        .call()
    {
        Err(err) => Err(err.to_string()),
        Ok(response) => match response.into_string() {
            Err(err) => Err(err.to_string()),
            Ok(body) => {
                let response: Result<LatestResponse, Error> = serde_json::from_str(&body);
                match response {
                    Err(err) => Err(err.to_string()),
                    Ok(response) => {
                        Ok(response.response[0].price)
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::*;
    use serde_json::Error;

    use crate::forex::FXPrice;

    #[test]
    fn test_get_fx_rate() {
        let app_id = "HNCSs6HDhJHAhRD6p4aR";
        let base = "GBP";
        let quoted = "USD";
        match super::get_fx_rate(&app_id, &base, &quoted) {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(rate) => assert_ne!(rate, Decimal::ZERO, "zero rate"),
        }
    }

    #[test]
    fn test_deserialize() {
        let payload = r#"{"id":"39","o":"1.37952","h":"1.38154","l":"1.37884","c":"1.38097","ch":"+0.00145","cp":"+0.11%","t":"1634694598","s":"GBP\/USD","tm":"2021-10-20 01:49:58"}"#;
        let result: Result<FXPrice, Error> = serde_json::from_str(&payload.to_string());
        match result {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(_price) => {},
        }
    }
}
