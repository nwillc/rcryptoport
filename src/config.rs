use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::io::Write;
use std::path::{Path, PathBuf};

use dirs::home_dir;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::model;
use crate::model::Portfolio;

const CONFIG_FILE: &'static str = ".crypto_port.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub app_id: String,
    pub portfolio: Portfolio,
    pub values: HashMap<String, Decimal>,
}

fn get_config_file() -> Result<PathBuf, String> {
    return match home_dir() {
        None => Err("home_dir not available".to_string()),
        Some(path) => {
            let mut config_path = PathBuf::from(path);
            config_path.push(CONFIG_FILE);
            Ok(config_path)
        }
    };
}

pub fn get_config() -> Result<Configuration, String> {
    let path = get_config_file()?;
    match fs::read_to_string(&path) {
        Err(err) => Err(err.to_string()),
        Ok(json) => {
            let c: Result<Configuration, Error> = serde_json::from_str(&json);
            match c {
                Err(err) => Err(err.to_string()),
                Ok(config) => Ok(config),
            }
        }
    }
}

pub fn update_config(
    config: &Configuration,
    values: &HashMap<String, Decimal>,
) -> Result<(), String> {
    let new_config = Configuration {
        app_id: config.app_id.clone(),
        portfolio: config.portfolio.clone(),
        values: values.clone(),
    };
    match get_config_file() {
        Err(err) => Err(err.to_string()),
        Ok(path) => write_config(path, &new_config),
    }
}

pub fn setup() {
    let app_id = read_string("Enter App ID: ".to_string());
    println!("Enter your crypto holdings, the currency name and the holding size. Blank currency when done.");
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
    let config = Configuration {
        app_id: app_id.to_string(),
        portfolio,
        values: Default::default(),
    };
    let path = get_config_file().expect("Unable to get config file");
    write_config(path, &config).expect("Unable to write config");
}

fn write_config<P: AsRef<Path>>(path: P, config: &Configuration) -> Result<(), String> {
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

    #[test]
    fn test_get_config_file() {
        match super::get_config_file() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(config_file) => assert_eq!(
                Some(OsStr::new(super::CONFIG_FILE)),
                config_file.file_name()
            ),
        }
    }

    #[test]
    fn test_get_config() {
        match super::get_config() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(_) => {}
        }
    }

    #[test]
    fn test_write_config() {
        match super::get_config() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(config) => match super::write_config("/tmp/foo.json", &config) {
                Err(err) => assert!(false, "unexpected error {}", err),
                Ok(_) => {}
            },
        }
    }

    // #[test]
    // fn test_update_config() {
    //     match super::get_config() {
    //         Err(err) => assert!(false, "unexpected error {}", err),
    //         Ok(config) => {
    //             let mut values: HashMap<String, Decimal> = HashMap::new();
    //             values.insert("FOO".to_string(), Decimal::ZERO);
    //             match super::update_config(&config, &values) {
    //                 Err(err) => assert!(false, "unexpected error {}", err),
    //                 Ok(_) => {}
    //             }
    //         }
    //     }
    // }
}
