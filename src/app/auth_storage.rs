use anyhow::{anyhow, Context, Result};
use argon2::{Argon2, Version, Params};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use rusqlite::{Connection, params, OpenFlags};
use std::path::Path;
use std::sync::Mutex;
use std::process::Command;
use std::fs;

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

pub struct AuthStorage {
    conn: Mutex<Connection>,
}

impl AuthStorage {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_CREATE | 
            OpenFlags::SQLITE_OPEN_READ_WRITE |
            OpenFlags::SQLITE_OPEN_NO_MUTEX  
        ).context("Failed to open credentials database")?;
        
        conn.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA foreign_keys=ON;
            PRAGMA secure_delete=ON;
        ").context("Failed to set database pragmas")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS credentials (
                id TEXT PRIMARY KEY,
                encrypted_data BLOB NOT NULL,
                nonce BLOB NOT NULL, 
                salt BLOB NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).context("Failed to create credentials table")?;
        
        Ok(Self { conn: Mutex::new(conn) })
    }

    fn generate_machine_key() -> Result<Vec<u8>> {
        #[cfg(target_os = "macos")]
        {
            let output = Command::new("sh")
                .arg("-c")
                .arg("ioreg -d2 -c IOPlatformExpertDevice | awk -F\\\" '/IOPlatformUUID/{print $(NF-1)}'")
                .output()
                .context("Failed to execute ioreg command")?;
            
            if !output.status.success() {
                return Err(anyhow!("ioreg command failed"));
            }
            
            let uuid = String::from_utf8(output.stdout)
                .context("Invalid UTF-8 output from ioreg")?
                .trim()
                .to_string();
                
            Ok(uuid.into_bytes())
        }
        
        #[cfg(target_os = "linux")]
        {
            let machine_id = if let Ok(id) = fs::read_to_string("/etc/machine-id") {
                id
            } else if let Ok(id) = fs::read_to_string("/var/lib/dbus/machine-id") {
                id
            } else {
                return Err(anyhow!("Could not read machine-id from either /etc/machine-id or /var/lib/dbus/machine-id"));
            };
            
            Ok(machine_id.trim().into_bytes())
        }
        
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("wmic")
                .args(&["csproduct", "get", "UUID"])
                .output()
                .context("Failed to execute wmic command")?;
                
            if !output.status.success() {
                return Err(anyhow!("wmic command failed"));
            }
            
            let uuid = String::from_utf8(output.stdout)
                .context("Invalid UTF-8 output from wmic")?
                .lines()
                .nth(1) // Skip header line
                .ok_or_else(|| anyhow!("No UUID found in wmic output"))?
                .trim()
                .to_string();
                
            Ok(uuid.into_bytes())
        }
    }

    fn derive_key(salt: &[u8], machine_key: &[u8]) -> Result<Vec<u8>> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Params::new(32768, 3, 4, None)
                .context("Failed to create Argon2 params")?
        );
        
        let mut output_key = vec![0u8; 32];
        argon2.hash_password_into(
            machine_key,
            salt,
            &mut output_key
        ).context("Key derivation failed")?;
        
        Ok(output_key)
    }

    pub fn store_credentials(&self, id: &str, password: &str) -> Result<()> {
        let mut salt = vec![0u8; SALT_SIZE];
        let mut nonce = vec![0u8; NONCE_SIZE];
        
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
        
        let conn = self.conn.lock()
            .map_err(|e| anyhow!("Failed to acquire database lock: {}", e))?;
            
        conn.execute(
            "INSERT OR REPLACE INTO credentials (id, encrypted_data, nonce, salt) VALUES (?1, ?2, ?3, ?4)",
            params![id, encrypted_data, nonce.as_slice(), salt],
        ).context("Failed to store encrypted credentials")?;

        Ok(())
    }

    pub fn get_credentials(&self, id: &str) -> Result<String> {
        let conn = self.conn.lock()
            .map_err(|e| anyhow!("Failed to acquire database lock: {}", e))?;
            
        let (encrypted_data, nonce, salt) = conn.query_row(
            "SELECT encrypted_data, nonce, salt FROM credentials WHERE id = ?1",
            params![id],
            |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    row.get::<_, Vec<u8>>(2)?,
                ))
            }
        ).with_context(|| format!("Failed to retrieve credentials for {}", id))?;

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

    pub fn verify_credentials(&self, id: &str, password: &str) -> Result<bool> {
        let stored_password = self.get_credentials(id)?;
        Ok(stored_password == password)
    }
}
