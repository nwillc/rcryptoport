use serde::{Deserialize};
use std::collections::HashMap;
use crate::model::Portfolio;
use rust_decimal::Decimal;
use std::path::PathBuf;
use dirs::home_dir;
use std::fs;
use serde_json::Error;

const CONFIG_FILE: &'static str = ".crypto_port.json";

#[derive(Deserialize, Debug)]
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
    }
}

pub fn get_config() -> Result<Configuration, String> {
    let path = get_config_file()?;
    match fs::read_to_string(&path) {
        Err(err) => Err(err.to_string()),
        Ok(json) => {
            let c : Result<Configuration, Error> = serde_json::from_str(&json);
            match c {
                Err(err) => Err(err.to_string()),
                Ok(config) => Ok(config)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    #[test]
    fn test_get_config_file() {
        match super::get_config_file() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(config_file) => assert_eq!(Some(OsStr::new(super::CONFIG_FILE)), config_file.file_name()),
        }
    }

    #[test]
    fn test_get_config() {
        match super::get_config() {
            Err(err) => assert!(false, "unexpected error {}", err),
            Ok(_) => {}
        }
    }
}
