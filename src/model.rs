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
