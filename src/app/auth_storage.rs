use anyhow::{anyhow, Result};
use argon2::Argon2;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use rusqlite::{Connection, params};
use std::path::Path;
use sysinfo::{System, SystemExt};

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

pub struct AuthStorage {
    conn: Connection,
}

impl AuthStorage {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS credentials (
                id TEXT PRIMARY KEY,
                encrypted_data BLOB NOT NULL,
                nonce BLOB NOT NULL,
                salt BLOB NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    fn generate_machine_key() -> Result<Vec<u8>> {
        let sys = System::new_all();
        let hostname = sys.host_name().unwrap_or_default();
        let os = sys.os_version().unwrap_or_default();
        
        // Create a unique machine identifier
        let machine_id = format!("{}:{}", hostname, os);
        Ok(machine_id.into_bytes())
    }

    fn derive_key(salt: &[u8], machine_key: &[u8]) -> Result<Vec<u8>> {
        let argon2 = Argon2::default();
        
        let mut output_key = vec![0u8; 32];  // 256-bit key for AES-256
        argon2.hash_password_into(
            machine_key,
            salt,
            &mut output_key
        )
        .map_err(|e| anyhow!("Key derivation failed: {}", e))?;
        
        Ok(output_key)
    }

    pub fn store_credentials(&self, id: &str, password: &str) -> Result<()> {
        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce = vec![0u8; NONCE_SIZE];
        
        // Generate random salt and nonce
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);
        
        let machine_key = Self::generate_machine_key()?;
        let key = Self::derive_key(&salt, &machine_key)?;
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
        
        let nonce = Nonce::from_slice(&nonce);
        let encrypted_data = cipher
            .encrypt(nonce, password.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        self.conn.execute(
            "INSERT OR REPLACE INTO credentials (id, encrypted_data, nonce, salt) VALUES (?1, ?2, ?3, ?4)",
            params![id, encrypted_data, nonce.as_slice(), salt],
        )?;
        
        Ok(())
    }

    pub fn verify_credentials(&self, id: &str, password: &str) -> Result<bool> {
        let stored_password = self.get_credentials(id)?;
        Ok(stored_password == password)
    }

    pub fn get_credentials(&self, id: &str) -> Result<String> {
        let mut stmt = self.conn.prepare(
            "SELECT encrypted_data, nonce, salt FROM credentials WHERE id = ?1"
        )?;
        
        let (encrypted_data, nonce, salt) = stmt.query_row(params![id], |row| {
            Ok((
                row.get::<_, Vec<u8>>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;
        
        let machine_key = Self::generate_machine_key()?;
        let key = Self::derive_key(&salt, &machine_key)?;
        
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| anyhow!("Failed to create cipher: {}", e))?;
        
        let nonce = Nonce::from_slice(&nonce);
        let decrypted = cipher
            .decrypt(nonce, encrypted_data.as_ref())
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        Ok(String::from_utf8(decrypted)?)
    }
}

