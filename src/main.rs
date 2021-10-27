use std::{thread, time};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Mul, Sub};
use std::path::PathBuf;

use chrono::{Duration, Utc};
use colored::*;
use jemallocator;
use rust_decimal::Decimal;

use crate::model::Configuration;

mod cli;
mod config;
mod model;
mod prices;
mod forex;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let matches = cli::cli();
    let quoted = if let Some(ref currency) = matches.value_of(cli::QUOTED) {
        currency
    } else {
        "USD"
    };

    let config_path = if let Some(ref config) = matches.value_of(cli::CONFIG) {
        PathBuf::from(config)
    } else {
        config::get_default_config_file().expect("unable to find default config")
    };

    if let Some(ref _matches) = matches.subcommand_matches(cli::SETUP) {
        config::setup();
        return;
    }

    let (looping, wait) = if let Some(ref seconds) = matches.value_of(cli::LOOP) {
        let wait_seconds: u64 = seconds.parse::<u64>().unwrap();
        (true, time::Duration::from_secs(wait_seconds))
    } else {
        (false, time::Duration::ZERO)
    };

    let mut config = config::get_config(&config_path).expect("unable to read config file");
    loop {
        let current_prices = print(&config, quoted);
        if !matches.is_present(cli::DRY_RUN) {
            config = Configuration {
                app_id: config.app_id.clone(),
                fx_app_id: config.fx_app_id.clone(),
                portfolio: config.portfolio.clone(),
                prices: current_prices.clone(),
                timestamp: Utc::now(),
            };
            config::write_config(&config_path, &config).expect("unable to update config")
        }
        if !looping {
            break;
        }
        thread::sleep(wait);
    }
}

fn print(configuration: &Configuration, quoted: &str) -> HashMap<String, Decimal> {
    let exchange_rate = if quoted == "USD" {
        Decimal::from(1)
    } else {
        forex::get_fx_rate("HNCSs6HDhJHAhRD6p4aR", "USD", quoted).unwrap()
    };
    let currencies: Vec<String> = configuration
        .portfolio
        .positions
        .iter()
        .map(|position| position.currency.to_string())
        .collect();
    let current_prices =
        prices::prices(&configuration.app_id, &currencies).expect("unable to retrieve prices");
    let mut prior_portfolio_value = Decimal::ZERO;
    let mut current_portfolio_value = Decimal::ZERO;

    println!(
        "{}",
        "Symbol     Price       Change         Holding             Position   Change".bold()
    );
    for position in configuration.portfolio.positions.iter() {
        let mut text: String;

        // symbol
        text = format!("{:>4}", position.currency);

        // current price
        let current_price_usd = match current_prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };
        let current_price_quoted = current_price_usd.mul(exchange_rate).round_dp(2);
        text += format!("{:>16}", current_price_quoted.to_string()).as_str();

        // prior price
        let prior_price_usd = match configuration.prices.get(&position.currency) {
            None => &Decimal::ZERO,
            Some(price) => price,
        };
        let prior_price_quoted = prior_price_usd.mul(exchange_rate).round_dp(2);
        // price change
        text += format!(
            " ({:>8})",
            current_price_quoted.sub(prior_price_quoted).mul(exchange_rate).round_dp(2).to_string())
            .as_str();

        // holding
        let holding_string = if position.holding > Decimal::ZERO {
            position.holding.to_string()
        } else {
            "".to_string()
        };
        text += format!(" {:>14}", holding_string).as_str();
        // position values
        let prior_value = position.holding.mul(prior_price_quoted).mul(exchange_rate);
        let current_value = position.holding.mul(current_price_quoted).mul(exchange_rate);

        if position.holding > Decimal::ZERO {
            text += format!(
                "{:>20} ({:>8})",
                current_value.round_dp(2).to_string(),
                (current_value - prior_value).round_dp(2).to_string()
            )
                .as_str();
        };

        // color
        let color = change_color(&prior_price_quoted, &current_price_quoted);
        println!("{}", text.color(color));
        prior_portfolio_value += prior_value;
        current_portfolio_value += current_value;
    }
    let color = change_color(&prior_portfolio_value, &current_portfolio_value);
    let since = Utc::now().signed_duration_since(configuration.timestamp);
    print!(
        "{} {}{:>28}",
        "Elapsed:".bold(),
        hhhmmss(since),
        "Total:".bold()
    );
    let text = format!(
        "{:>20} ({:>8})",
        current_portfolio_value.round_dp(2),
        (current_portfolio_value - prior_portfolio_value)
            .round_dp(2)
            .to_string()
    );
    println!("{}", text.color(color));
    return current_prices;
}

fn hhhmmss(duration: Duration) -> String {
    let sec = duration.num_seconds();
    let seconds = sec % 60;
    let minutes = (sec / 60) % 60;
    let hours = sec / 60 / 60;
    format!("{:03}:{:02}:{:02}", hours, minutes, seconds)
}

#[inline]
fn change_color(prior: &Decimal, current: &Decimal) -> String {
    match Decimal::ZERO.cmp(&current.sub(prior)) {
        Ordering::Greater => "red",
        Ordering::Less => "green",
        Ordering::Equal => "white",
    }
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::Duration;
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
        super::print(&configuration, "USD");
    }

    #[test]
    fn test_hhhmmss() {
        let d = Duration::seconds(10);
        assert_eq!("000:00:10", super::hhhmmss(d));
        let d = Duration::seconds(61);
        assert_eq!("000:01:01", super::hhhmmss(d));
        let d = Duration::minutes(10);
        assert_eq!("000:10:00", super::hhhmmss(d));
        let d = Duration::minutes(10) + Duration::seconds(10);
        assert_eq!("000:10:10", super::hhhmmss(d));
        let d = Duration::hours(10);
        assert_eq!("010:00:00", super::hhhmmss(d));
        let d = Duration::hours(10) + Duration::minutes(10) + Duration::seconds(10);
        assert_eq!("010:10:10", super::hhhmmss(d));
    }
}
