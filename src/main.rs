use std::collections::HashMap;

use colored::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

mod config;
mod model;
mod prices;

fn main() {
    let config = config::get_config().expect("unable to read config file");
    let currencies: Vec<String> = config
        .portfolio
        .positions
        .iter()
        .map(|position| position.currency.to_string())
        .collect();
    let prices = prices::prices(&config.app_id, &currencies).expect("unable to retrieve prices");
    let mut current_total: Decimal = Decimal::from(0);
    let mut current_values: HashMap<String, Decimal> = HashMap::new();
    for position in config.portfolio.positions.iter() {
        match prices.get(&position.currency) {
            None => {}
            Some(price) => {
                let current_value = position.holding * price;
                let mut prior_value: Decimal = Decimal::ZERO;
                match config.values.get(&position.currency) {
                    None => {}
                    Some(value) => prior_value = *value,
                }
                let percent_change = percent_change(prior_value, current_value);
                let color = change_color(percent_change);
                let text = format!(
                    "{:16} {:4} {:13} ({:>7}%)",
                    position.holding,
                    position.currency,
                    current_value.round_dp(2),
                    percent_change.round_dp(2)
                );
                println!("{}", text.color(color));
                current_values.insert(position.currency.clone(), current_value);
                current_total += current_value;
            }
        }
    }
    let mut prior_total: Decimal = Decimal::from(0);
    for (_key, value) in config.values.iter() {
        prior_total += value;
    }
    let percent_change = percent_change(prior_total, current_total);
    let color = change_color(percent_change);
    let text = format!(
        "{:>20} {:14} ({:>7}%)",
        "Total:",
        current_total.round_dp(2),
        percent_change.round_dp(2)
    );
    println!("{}", text.color(color));
    config::update_config(&config, &current_values).expect("unable to update config")
}

fn percent_change(prior: Decimal, current: Decimal) -> Decimal {
    (current - prior) / prior * Decimal::from(100)
}

fn change_color(change: Decimal) -> String {
    match change.to_f32() {
        Some(f) if f < 0.0 => "red",
        Some(f) if f > 0.0 => "green",
        _ => "white",
    }
    .to_string()
}
