use crate::config::Config;
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

use reqwest::Client;
use serde::Deserialize;

use crate::utils;

#[derive(Deserialize)]
struct TokenResponse {
    Token: String,
}

#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub obtained_at: u64,
    pub expires_at: u64,
}

pub async fn authenticate(config: &Config) -> Result<Token> {
    // Check for existing token
    if let Some(token) = load_token()? {
        if token.expires_at > utils::current_timestamp() {
            // Token is still valid
            return Ok(token);
        }
    }

    // Token is missing or expired; proceed to authenticate
    let credentials = load_credentials(&config.username)?;
    let password = prompt_password(&config.username)?;
    if !verify_password(&password, &credentials.password_hash)? {
        return Err(anyhow!("Invalid password"));
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let auth_url = format!("{}/dna/system/api/v1/auth/token", config.dnac_url);

    let resp = client
        .post(&auth_url)
        .basic_auth(&config.username, Some(&password))
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Authentication failed with status: {}",
            resp.status()
        ));
    }

    let token_resp: TokenResponse = resp.json().await?;

    let obtained_at = utils::current_timestamp();
    let expires_at = obtained_at + 1 * 60 * 60; // Token valid for 1 hour

    let token = Token {
        value: token_resp.Token,
        obtained_at,
        expires_at,
    };

    // Store the token
    store_token(&token)?;

    Ok(token)
}

struct StoredCredentials {
    password_hash: String,
}

fn get_db_path() -> PathBuf {
    let mut db_path = dirs::config_dir().unwrap();
    db_path.push("dnac");
    db_path.push("credentials.db");
    db_path
}

fn load_credentials(username: &str) -> Result<StoredCredentials> {
    let db_path = get_db_path();
    if !db_path.exists() {
        println!("No credentials database found. Starting setup...");
        store_credentials(username)?;
    }

    let conn = Connection::open(db_path)?;

    // Ensure the credentials and token tables exist
    create_tables(&conn)?;

    let mut stmt = conn.prepare("SELECT password_hash FROM credentials WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        let password_hash: String = row.get(0)?;
        Ok(StoredCredentials { password_hash })
    } else {
        println!("No credentials found for user '{}'. Starting setup...", username);
        store_credentials(username)?;

        // After storing credentials, try to load them again
        let mut stmt = conn.prepare("SELECT password_hash FROM credentials WHERE username = ?1")?;
        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            let password_hash: String = row.get(0)?;
            Ok(StoredCredentials { password_hash })
        } else {
            Err(anyhow!("Credentials not found after setup"))
        }
    }
}

fn store_credentials(username: &str) -> Result<()> {
    let password = prompt_new_password(username)?;

    let db_path = get_db_path();
    fs::create_dir_all(db_path.parent().unwrap())?;
    let conn = Connection::open(db_path)?;

    // Create the necessary tables
    create_tables(&conn)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e))?
        .to_string();

    conn.execute(
        "INSERT INTO credentials (username, password_hash) VALUES (?1, ?2)",
        params![username, password_hash],
    )?;

    Ok(())
}

fn prompt_password(username: &str) -> Result<String> {
    let password = rpassword::prompt_password(format!("Enter password for {}: ", username))?;
    Ok(password)
}

fn prompt_new_password(username: &str) -> Result<String> {
    let password = rpassword::prompt_password(format!("Set a new password for {}: ", username))?;
    let confirm_password = rpassword::prompt_password("Confirm password: ")?;

    if password != confirm_password {
        return Err(anyhow!("Passwords do not match"));
    }

    Ok(password)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| anyhow!(e))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// Functions to store and load the token
fn store_token(token: &Token) -> Result<()> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;

    // Create the necessary tables
    create_tables(&conn)?;

    conn.execute(
        "DELETE FROM token", // Clear any existing token
        [],
    )?;

    conn.execute(
        "INSERT INTO token (value, obtained_at, expires_at) VALUES (?1, ?2, ?3)",
        params![token.value, token.obtained_at, token.expires_at],
    )?;

    Ok(())
}

fn load_token() -> Result<Option<Token>> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;

    // Create the necessary tables
    create_tables(&conn)?;

    let mut stmt = conn.prepare("SELECT value, obtained_at, expires_at FROM token LIMIT 1")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        let obtained_at: u64 = row.get(1)?;
        let expires_at: u64 = row.get(2)?;

        Ok(Some(Token {
            value,
            obtained_at,
            expires_at,
        }))
    } else {
        Ok(None)
    }
}

// Helper function to create necessary tables
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS token (
            id INTEGER PRIMARY KEY,
            value TEXT NOT NULL,
            obtained_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}
