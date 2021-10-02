use std::collections::HashMap;
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
    pub app_id: String,
    pub portfolio: Portfolio,
    pub prices: HashMap<String, Decimal>,
}
