use std::collections::HashMap;
use std::ops::{Mul, Sub};

use clap::App;
use colored::*;
use jemallocator;
use rust_decimal::Decimal;

mod config;
mod model;
mod prices;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const SETUP: &str = "setup";

fn main() {
    let matches = App::new("rcryptoport")
        .version("1.0")
        .author("nwillc@gmail.com")
        .about("Retrieve current value of your crypto portfolio.")
        .subcommand(App::new(SETUP)
            .about("Set up portfolio configuration"))
        .get_matches();

    if let Some(ref _matches) = matches.subcommand_matches(SETUP) {
        config::setup();
        return;
    }
    let config_path = config::get_default_config_file().expect("unable to find default config");
    let config = config::get_config(&config_path).expect("unable to read config file");
    let current_prices = print(&config);
    config::update_config(config_path, &config, &current_prices).expect("unable to update config")
}

fn print(configuration: &config::Configuration) -> HashMap<String, Decimal> {
    let currencies: Vec<String> = configuration
        .portfolio
        .positions
        .iter()
        .map(|position| position.currency.to_string())
        .collect();
    let current_prices = prices::prices(&configuration.app_id, &currencies).expect("unable to retrieve prices");
    let mut prior_portfolio_value = Decimal::ZERO;
    let mut current_portfolio_value = Decimal::ZERO;
    for position in configuration.portfolio.positions.iter() {
        let holding_string = if position.holding > Decimal::ZERO {
            position.holding.to_string()
        } else {
            "".to_string()
        };
        let mut text = format!("{:16} {:4}", holding_string, position.currency);
        let prior_price = match configuration.prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };
        text += format!("{:>16}", prior_price.to_string()).as_str();
        let current_price = match current_prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };
        text += format!("{:>16}", current_price.to_string()).as_str();
        let color = change_color(prior_price, current_price);
        text += format!(" ({:>8})", (current_price - prior_price).round_dp(2).to_string()).as_str();
        let prior_value = position.holding.mul(prior_price);
        let current_value = position.holding.mul(current_price);
        let current_value_str = if position.holding > Decimal::ZERO {
            current_value.round_dp(2).to_string()
        } else {
            "".to_string()
        };
        text += format!("{:>20}", current_value_str).as_str();
        println!("{}", text.color(color));
        prior_portfolio_value += prior_value;
        current_portfolio_value += current_value;
    }
    let color = change_color(&prior_portfolio_value, &current_portfolio_value);
    let text = format!("{:>64}{:>20}", "Total:", current_portfolio_value.round_dp(2));
    println!("{}", text.color(color));
    return current_prices;
}

fn change_color(prior: &Decimal, current: &Decimal) -> String {
    let delta = current.sub(prior);
    let color = if delta < Decimal::ZERO {
        "red"
    } else if delta > Decimal::ZERO {
        "green"
    } else {
        "white"
    };
    return color.to_string();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config;

    #[test]
    fn test_print() {
        let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_file.push("testdata/portfolio2.json");
        let configuration = config::get_config(config_file).expect("unable to read config");
        super::print(&configuration);
    }
}
