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
use sysinfo::{System, SystemExt, CpuExt};

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

/// AuthStorage handles secure storage and retrieval of encrypted credentials
pub struct AuthStorage {
    conn: Mutex<Connection>,
}

impl AuthStorage {
    /// Create a new AuthStorage instance with the given database path
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        // Open database with WAL mode and correct permissions
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_CREATE | 
            OpenFlags::SQLITE_OPEN_READ_WRITE |
            OpenFlags::SQLITE_OPEN_NO_MUTEX  
        ).context("Failed to open credentials database")?;
        
        // Enable WAL mode and foreign keys
        conn.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA foreign_keys=ON;
            PRAGMA secure_delete=ON;
        ").context("Failed to set database pragmas")?;
        
        // Create credentials table if it doesn't exist
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
        let mut sys = System::new_all();
        sys.refresh_all();
        
        // Get system info with fallbacks
        let hostname = sys.host_name()
            .context("Failed to get hostname")?;
        let os = sys.os_version()
            .context("Failed to get OS version")?;
        let cpu_info = sys.cpus().first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_default();
        
        // Create a unique machine identifier combining multiple factors
        let machine_id = format!("{}:{}:{}", hostname, os, cpu_info);        
        Ok(machine_id.into_bytes())
    }

    fn derive_key(salt: &[u8], machine_key: &[u8]) -> Result<Vec<u8>> {
        // Configure Argon2 with consistent parameters
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            Version::V0x13,
            Params::new(32768, 3, 4, None)
                .context("Failed to create Argon2 params")?
        );
        
        let mut output_key = vec![0u8; 32];  // 256-bit key for AES-256
        argon2.hash_password_into(
            machine_key,
            salt,
            &mut output_key
        )
        .context("Key derivation failed")?;
        
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
        
        // Acquire mutex lock for database access
        let conn = self.conn.lock()
            .map_err(|e| anyhow!("Failed to acquire database lock: {}", e))?;
            
        conn.execute(
            "INSERT OR REPLACE INTO credentials (id, encrypted_data, nonce, salt) VALUES (?1, ?2, ?3, ?4)",
            params![id, encrypted_data, nonce.as_slice(), salt],
        ).context("Failed to store encrypted credentials")?;

        Ok(())
    }

    pub fn verify_credentials(&self, id: &str, password: &str) -> Result<bool> {
        let stored_password = self.get_credentials(id)?;
        Ok(stored_password == password)
    }

    pub fn get_credentials(&self, id: &str) -> Result<String> {
        // Acquire mutex lock for database access
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
}

