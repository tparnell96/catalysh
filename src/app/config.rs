use anyhow::Result;
use rpassword;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use dirs::config_dir;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialMode {
    /// Credentials are encrypted and stored on-device; no login prompt on each launch.
    #[default]
    StoreOnDevice,
    /// Credentials are never persisted; the user is prompted for a password on each
    /// launch (or whenever the cached token has expired).
    SessionOnly,
}

impl fmt::Display for CredentialMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CredentialMode::StoreOnDevice => write!(f, "store-on-device"),
            CredentialMode::SessionOnly => write!(f, "session-only"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub dnac_url: String,
    pub username: String,
    pub verify_ssl: bool,
    #[serde(default)]
    pub credential_mode: CredentialMode,
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = config_dir().expect("Could not determine config directory");
    config_path.push("catalysh");
    fs::create_dir_all(&config_path).expect("Could not create config directory");
    config_path.push("config.yml");
    config_path
}

pub fn get_credentials_db_path() -> PathBuf {
    let mut db_path = config_dir().expect("Could not determine config directory");
    db_path.push("catalysh");
    db_path.push("credentials.db");
    db_path
}

/// Returns the directory where custom CA/SSL certificates are stored.
pub fn get_certs_dir() -> PathBuf {
    let mut dir = config_dir().expect("Could not determine config directory");
    dir.push("catalysh");
    dir.push("certs");
    dir
}

/// Copy a PEM/CRT/CER certificate file into the catalysh certs directory.
/// Returns the name it was stored under.
pub fn install_cert(source: &std::path::Path) -> Result<String> {
    let name = source
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Could not determine certificate filename"))?
        .to_string();

    let certs_dir = get_certs_dir();
    fs::create_dir_all(&certs_dir)?;

    // Basic sanity check: file must be readable
    let contents = fs::read(source)
        .map_err(|e| anyhow::anyhow!("Could not read certificate file '{}': {}", source.display(), e))?;

    if contents.is_empty() {
        return Err(anyhow::anyhow!("Certificate file is empty"));
    }

    let dest = certs_dir.join(&name);
    fs::write(&dest, &contents)
        .map_err(|e| anyhow::anyhow!("Could not write certificate to '{}': {}", dest.display(), e))?;

    Ok(name)
}

/// List the names of all installed custom certificates.
pub fn list_certs() -> Result<Vec<String>> {
    let certs_dir = get_certs_dir();
    if !certs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(&certs_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy().to_string();
        // Skip hidden files and non-cert extensions
        if !name.starts_with('.') {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

/// Remove a named certificate from the certs directory.
pub fn remove_cert(name: &str) -> Result<()> {
    let cert_path = get_certs_dir().join(name);
    if !cert_path.exists() {
        return Err(anyhow::anyhow!("Certificate '{}' not found", name));
    }
    fs::remove_file(&cert_path)?;
    Ok(())
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

/// Update the credential storage mode
pub fn update_credential_mode(mode: CredentialMode) -> Result<()> {
    let mut config = load_config()?;

    if mode == CredentialMode::SessionOnly && config.credential_mode == CredentialMode::StoreOnDevice {
        // Wipe any stored credentials so they are not left on disk
        let db_path = get_credentials_db_path();
        if db_path.exists() {
            fs::remove_file(&db_path)?;
            println!("Stored credentials removed from disk.");
        }
    }

    config.credential_mode = mode;
    save_config(&config)?;
    println!("Credential mode updated to: {}", config.credential_mode);
    Ok(())
}

/// Reset only the stored credentials while keeping other settings
pub fn reset_credentials() -> Result<()> {
    let config = load_config()?;

    let credentials_db_path = get_credentials_db_path();
    if credentials_db_path.exists() {
        fs::remove_file(&credentials_db_path)?;
        println!("Previous credentials have been removed.");
    }

    if config.credential_mode == CredentialMode::SessionOnly {
        println!("Credential mode is session-only; no credentials will be stored.");
        return Ok(());
    }

    // Prompt for updated username
    let mut config = config;
    print!("Enter username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    config.username = username.trim().to_string();
    save_config(&config)?;

    let password = rpassword::prompt_password("Enter new password: ")?;

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
    let mut credential_mode_input = String::new();

    print!("Enter Cisco DNAC URL without a / at the end (e.g., https://dnac.example.com, https://192.168.1.20): ");
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

    print!("Store credentials on this device? (y = store encrypted on-device, n = prompt each session) [y/n]: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut credential_mode_input)?;
    let credential_mode = if credential_mode_input.trim().to_lowercase() == "n" {
        println!("Session-only mode selected. You will be prompted for your password each time the token expires.");
        CredentialMode::SessionOnly
    } else {
        CredentialMode::StoreOnDevice
    };

    let config = Config {
        dnac_url,
        username: username.clone(),
        verify_ssl,
        credential_mode: credential_mode.clone(),
    };

    if credential_mode == CredentialMode::StoreOnDevice {
        let password = rpassword::prompt_password("Enter your password: ")?;
        let auth_storage = crate::app::auth_storage::AuthStorage::new(get_credentials_db_path())?;
        match auth_storage.store_credentials(&username, &password) {
            Ok(_) => println!("Credentials stored securely."),
            Err(e) => return Err(anyhow::anyhow!("Failed to store credentials: {}", e)),
        }
        println!("Configuration complete. Credentials stored securely.");
    } else {
        println!("Configuration complete. You will be prompted for your password when needed.");
    }

    Ok(config)
}

/// Save the configuration to a file
fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    let contents = serde_yaml::to_string(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}
