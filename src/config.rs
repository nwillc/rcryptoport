use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::io::Write;
use std::path::{Path, PathBuf};

use dirs::home_dir;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
// use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::model;

const CONFIG_FILE: &'static str = ".crypto_port.json";

pub fn get_default_config_file() -> Result<PathBuf, String> {
    return match home_dir() {
        None => Err("home_dir not available".to_string()),
        Some(path) => {
            let mut config_path = PathBuf::from(path);
            config_path.push(CONFIG_FILE);
            Ok(config_path)
        }
    };
}

pub fn get_config<P: AsRef<Path>>(path: P) -> Result<model::Configuration, String> {
    match fs::read_to_string(&path) {
        Err(err) => Err(err.to_string()),
        Ok(json) => {
            let c: Result<model::Configuration, Error> = serde_json::from_str(&json);
            match c {
                Err(err) => Err(err.to_string()),
                Ok(config) => Ok(config),
            }
        }
    }
}

pub fn write_config<P: AsRef<Path>>(path: P, app_id: String, portfolio: model::Portfolio, prices: HashMap<String, Decimal>) -> Result<(), String> {
    let config = model::Configuration { app_id, portfolio, prices };
    match serde_json::to_string(&config) {
        Err(err) => Err(err.to_string()),
        Ok(as_json) => match fs::File::create(path) {
            Err(err) => Err(err.to_string()),
            Ok(mut file) => match file.write_all(as_json.as_bytes()) {
                Err(err) => Err(err.to_string()),
                Ok(_) => Ok(()),
            },
        },
    }
}

pub fn setup() {
    let app_id = read_string("Enter App ID: ".to_string());
    println!("Enter your crypto holdings, the currency name and the holding size. Holding can be zero. Enter return in currency when done.");
    let mut positions: Vec<model::Position> = Vec::new();
    loop {
        let mut currency = read_string(" Currency: ".to_string());
        if currency.eq("") {
            break;
        }
        currency.make_ascii_uppercase();
        let holding = read_string(" Holding: ".to_string());
        match Decimal::from_str(holding.as_str()) {
            Err(err) => println! {"Invalid holding: {}", err},
            Ok(value) => {
                let position = model::Position { currency, holding: value };
                positions.push(position);
            }
        }
    };
    let portfolio = model::Portfolio { positions };
    let path = get_default_config_file().expect("Unable to get config file");
    write_config(path, app_id.to_string(), portfolio, Default::default()).expect("Unable to write config");
}

fn read_string(prompt: String) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).expect("Could not read line");
    let len = line.trim_end().len();
    line.truncate(len);
    return line;
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::PathBuf;

    use temp_testdir::TempDir;

    #[test]
    fn test_get_default_config_file() {
        match super::get_default_config_file() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(config_file) => assert_eq!(
                Some(OsStr::new(super::CONFIG_FILE)),
                config_file.file_name()
            ),
        }
    }

    #[test]
    fn test_get_config() {
        let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_file.push("testdata/portfolio2.json");
        match super::get_config(config_file) {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(_) => {}
        }
    }

    #[test]
    fn test_write_config() {
        let temp = TempDir::default();
        let mut output_path = PathBuf::from(temp.as_ref());
        output_path.push("config.json");
        let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        config_file.push("testdata/portfolio2.json");
        match super::get_config(config_file) {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(config) => match super::write_config(output_path, config.app_id, config.portfolio, config.prices) {
                Err(err) => assert!(false, "unexpected error {}", err),
                Ok(_) => {}
            },
        }
    }
}
