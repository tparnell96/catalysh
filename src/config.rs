use anyhow::{Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use dirs::config_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub dnac_url: String,
    pub username: String,
    pub verify_ssl: bool,
}

impl Config {
    pub fn new(dnac_url: String, username: String, verify_ssl: bool) -> Self {
        Self {
            dnac_url,
            username,
            verify_ssl,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = config_dir().unwrap();
    config_path.push("dnac");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.yml");
    config_path
}

pub fn get_credentials_db_path() -> PathBuf {
    let mut db_path = config_dir().unwrap();
    db_path.push("dnac");
    db_path.push("credentials.db");
    db_path
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    } else {
        println!("Config file not found. Starting setup...");
        let config = setup_config()?;
        save_config(&config)?;
        Ok(config)
    }
}

pub fn reset_config() -> Result<()> {
    let config_path = get_config_path();
    let credentials_db_path = get_credentials_db_path();

    if config_path.exists() {
        fs::remove_file(config_path)?;
    }

    if credentials_db_path.exists() {
        fs::remove_file(credentials_db_path)?;
    }

    println!("Configuration files deleted.");
    Ok(())
}

fn setup_config() -> Result<Config> {
    let mut dnac_url = String::new();
    let mut username = String::new();
    let mut verify_ssl_input = String::new();

    print!("Enter Cisco DNAC URL without a / at the end(e.g., https://dnac.example.com, https://192.168.1.20): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut dnac_url)?;
    dnac_url = dnac_url.trim().to_string();

    print!("Enter your username: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;
    username = username.trim().to_string();

    print!("Verify SSL certificates? (y/n): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut verify_ssl_input)?;
    let verify_ssl = verify_ssl_input.trim().to_lowercase() == "y";

    Ok(Config::new(dnac_url, username, verify_ssl))
}

fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    let contents = serde_yaml::to_string(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}
