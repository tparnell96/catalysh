use anyhow::Result;
use rpassword;
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
    config_path.push("catalysh");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.yml");
    config_path
}

pub fn get_credentials_db_path() -> PathBuf {
    let mut db_path = config_dir().unwrap();
    db_path.push("catalysh");
    db_path.push("credentials.db");
    db_path
}

/// Load configuration and trigger setup if necessary
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    if config_path.exists() {
        let contents = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    } else {
        println!("Configuration file not found. Starting setup...");
        let config = setup_config()?;
        save_config(&config)?;
        Ok(config)
    }
}

/// Reset the configuration and credentials
pub fn reset_config() -> Result<()> {
    let config_path = get_config_path();
    let credentials_db_path = get_credentials_db_path();

    if config_path.exists() {
        fs::remove_file(config_path)?;
    }

    if credentials_db_path.exists() {
        fs::remove_file(credentials_db_path)?;
    }

    println!("Configuration files and credentials deleted.");
    Ok(())
}

/// Update the DNA Center URL in the configuration
pub fn update_dnac_url(url: String) -> Result<()> {
    let mut config = load_config()?;
    config.dnac_url = url;
    save_config(&config)?;
    println!("DNA Center URL updated successfully.");
    Ok(())
}

/// Update the SSL verification setting in the configuration
pub fn update_verify_ssl(verify: bool) -> Result<()> {
    let mut config = load_config()?;
    config.verify_ssl = verify;
    save_config(&config)?;
    println!("SSL verification setting updated successfully.");
    Ok(())
}

/// Reset only the stored credentials while keeping other settings
pub fn reset_credentials() -> Result<()> {
    let credentials_db_path = get_credentials_db_path();
    if credentials_db_path.exists() {
        fs::remove_file(credentials_db_path)?;
        println!("Previous credentials have been removed.");
    }
    
    // Load existing config
    let mut config = load_config()?;
    
    // Prompt for username
    print!("Enter username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    config.username = username.trim().to_string();
    
    // Save the new username to config
    save_config(&config)?;
    
    // Prompt for new password
    let password = rpassword::prompt_password("Enter new password: ")?;
    
    // Store new credentials
    let auth_storage = crate::app::auth_storage::AuthStorage::new(get_credentials_db_path())?;
    auth_storage.store_credentials(&config.username, &password)?;
    
    println!("New credentials have been stored.");
    Ok(())
}

/// Setup configuration by prompting the user
fn setup_config() -> Result<Config> {
    let mut dnac_url = String::new();
    let mut username = String::new();
    let mut verify_ssl_input = String::new();

    print!("Enter Cisco DNAC URL without a / at the end (e.g., https://dnac.example.com, https://192.168.1.20): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut dnac_url)?;
    dnac_url = dnac_url.trim().to_string();

    print!("Enter your username: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;
    username = username.trim().to_string();

    let password = rpassword::prompt_password("Enter your password: ")?;

    print!("Verify SSL certificates? (y/n): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut verify_ssl_input)?;
    let verify_ssl = verify_ssl_input.trim().to_lowercase() == "y";

    // Store password securely
    let auth_storage = crate::app::auth_storage::AuthStorage::new(get_credentials_db_path())?;
    // Store credentials with explicit username match
    match auth_storage.store_credentials(&username, &password) {
        Ok(_) => println!("Credentials stored securely."),
        Err(e) => return Err(anyhow::anyhow!("Failed to store credentials: {}", e))
    }
    println!("Configuration complete. Credentials stored securely.");

    Ok(Config::new(dnac_url, username, verify_ssl))
}

/// Save the configuration to a file
fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    let contents = serde_yaml::to_string(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}

