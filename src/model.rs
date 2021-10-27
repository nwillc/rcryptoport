use std::collections::HashMap;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub currency: String,
    pub holding: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub fx_app_id: String,
    #[serde(with = "ts_seconds", default = "chrono::Utc::now")]
    pub timestamp: DateTime<Utc>,
    pub portfolio: Portfolio,
    pub prices: HashMap<String, Decimal>,
}
