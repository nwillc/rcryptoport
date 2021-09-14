use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Mul, Sub};
use std::path::PathBuf;

use clap::{App, Arg};
use colored::*;
use jemallocator;
use rust_decimal::Decimal;
use std::{time, thread};

mod config;
mod model;
mod prices;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const CONFIG: &str = "config";
const DRY_RUN: &str = "dry-run";
const LOOP: &str = "loop";
const SETUP: &str = "setup";

fn main() {
    let matches = App::new("rcryptoport")
        .version("2.0.0")
        .author("nwillc@gmail.com")
        .about("Retrieve current value of your crypto portfolio.")
        .arg(Arg::with_name(CONFIG)
            .short("c")
            .long(CONFIG)
            .takes_value(true)
            .value_name("FILE")
            .help("Path to specific config file"))
        .arg(Arg::with_name(DRY_RUN)
            .short("d")
            .long(DRY_RUN)
            .help("Dry run, do not save values"))
        .arg(Arg::with_name(LOOP)
            .short("l")
            .long(LOOP)
            .takes_value(true)
            .value_name("SECONDS")
            .help("Run looping every SECONDS seconds"))
        .subcommand(App::new(SETUP)
            .about("Set up portfolio configuration"))
        .get_matches();

    let config_path: PathBuf;
    if let Some(ref config) = matches.value_of(CONFIG) {
        config_path = PathBuf::from(config)
    } else {
        config_path = config::get_default_config_file().expect("unable to find default config");
    }
    if let Some(ref _matches) = matches.subcommand_matches(SETUP) {
        config::setup();
        return;
    }
    let looping: bool;
    let wait: time::Duration;
    if let Some(ref seconds) = matches.value_of(LOOP) {
        looping = true;
        let wait_seconds: u64 = seconds.parse::<u64>().unwrap();
        wait = time::Duration::from_secs(wait_seconds);
    } else {
        looping = false;
        wait = time::Duration::ZERO;
    }
    let mut config = config::get_config(&config_path).expect("unable to read config file");
    loop {
        let current_prices = print(&config);
        if !matches.is_present(DRY_RUN) {
            config = config::Configuration {
                app_id: config.app_id.clone(),
                portfolio: config.portfolio.clone(),
                prices: current_prices.clone(),
            };
            config::write_config(&config_path, &config).expect("unable to update config")
        }
        if !looping {
            break;
        }
        thread::sleep(wait);
    }
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

    println!("{}", "Symbol     Price       Change         Holding             Position   Change".bold());
    for position in configuration.portfolio.positions.iter() {
        let mut text: String;

        // symbol
        text = format!("{:>4}", position.currency);

        // current price
        let current_price = match current_prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };
        text += format!("{:>16}", current_price.to_string()).as_str();

        // prior price
        let prior_price = match configuration.prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };

        // price change
        text += format!(" ({:>8})", (current_price - prior_price).round_dp(2).to_string()).as_str();

        // holding
        let holding_string = if position.holding > Decimal::ZERO {
            position.holding.to_string()
        } else {
            "".to_string()
        };
        text += format!(" {:>14}", holding_string).as_str();
        // holding values
        let prior_value = position.holding.mul(prior_price);
        let current_value = position.holding.mul(current_price);

        if position.holding > Decimal::ZERO {
            text += format!("{:>20} ({:>8})",
                            current_value.round_dp(2).to_string(),
                            (current_value - prior_value).round_dp(2).to_string()).as_str();
        };

        // color
        let color = change_color(&prior_price, &current_price);
        println!("{}", text.color(color));
        prior_portfolio_value += prior_value;
        current_portfolio_value += current_value;
    }
    let color = change_color(&prior_portfolio_value, &current_portfolio_value);
    print!("{:>45}", "Total:".bold());
    let text = format!("{:>21} ({:>8})",
                       current_portfolio_value.round_dp(2),
                       (current_portfolio_value - prior_portfolio_value).round_dp(2).to_string());
    println!("{}", text.color(color));
    return current_prices;
}

#[inline]
fn change_color(prior: &Decimal, current: &Decimal) -> String {
    match Decimal::ZERO.cmp(&current.sub(prior)) {
        Ordering::Greater => "red",
        Ordering::Less => "green",
        Ordering::Equal => "white"
    }.to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rust_decimal::Decimal;

    use crate::config;

    #[test]
    fn test_change_color() {
        let mut color: String;
        color = super::change_color(&Decimal::from(0), &Decimal::from(1));
        assert_eq!("green", color);
        color = super::change_color(&Decimal::from(1), &Decimal::from(0));
        assert_eq!("red", color);
        color = super::change_color(&Decimal::from(1), &Decimal::from(1));
        assert_eq!("white", color);
    }

    #[test]
    fn test_print() {
        let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_file.push("testdata/portfolio2.json");
        let configuration = config::get_config(config_file).expect("unable to read config");
        super::print(&configuration);
    }
}
