use crate::app::auth_storage::AuthStorage;
use crate::app::config::Config;
use crate::helpers::utils;
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};
use std::path::PathBuf;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct TokenResponse {
    Token: String,
}

#[derive(Clone)]
#[allow(non_snake_case)]
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
    let auth_storage = AuthStorage::new(get_db_path())?;

    // Get stored credentials
    let password = match auth_storage.get_credentials(&config.username) {
        Ok(pwd) => pwd,
        Err(_) => return Err(anyhow!("Could not retrieve stored credentials. Please run 'app config reset' and reconfigure."))
    };

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
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(anyhow!("Authentication failed: Invalid credentials. Please run 'app config reset' to reconfigure."));
        } else {
            return Err(anyhow!(
                "Authentication failed with status: {} - {}",
                resp.status(),
                resp.status().canonical_reason().unwrap_or("Unknown error")
            ));
        }
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

fn get_db_path() -> PathBuf {
    let mut db_path = dirs::config_dir().unwrap();
    db_path.push("catalysh");
    db_path.push("credentials.db");
    db_path
}
// Functions to store and load the token
fn store_token(token: &Token) -> Result<()> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;

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
