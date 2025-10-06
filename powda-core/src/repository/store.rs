use crate::domain::{PasswordEntry, EntryName};
use crate::error::{Error, Result};
use crate::crypto::{CryptoManager, EncryptedVault, encoding};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait StoreRepository: Send + Sync {
    async fn init(&self, master_password: &str) -> Result<()>;
    async fn unlock(&self, master_password: &str) -> Result<()>;
    async fn lock(&self) -> Result<()>;
    async fn is_locked(&self) -> bool;
    async fn exists(&self) -> bool;
    async fn add(&self, entry: PasswordEntry) -> Result<()>;
    async fn get(&self, name: &EntryName) -> Result<PasswordEntry>;
    async fn list(&self) -> Result<Vec<EntryName>>;
    async fn update(&self, entry: PasswordEntry) -> Result<()>;
    async fn remove(&self, name: &EntryName) -> Result<()>;
    async fn change_master_password(&self, current: &str, new: &str) -> Result<()>;
}

pub struct Store {
    path: PathBuf,
    crypto: Arc<Mutex<CryptoManager>>,
    cache: Arc<Mutex<Option<HashMap<String, PasswordEntry>>>>,
}

impl Store {
    pub fn new() -> Self {
        let home = std::env::var("HOME").expect("HOME not set");
        let path = PathBuf::from(home).join(".powda_vault.encrypted");
        Store {
            path,
            crypto: Arc::new(Mutex::new(CryptoManager::new())),
            cache: Arc::new(Mutex::new(None)),
}
    }

    pub fn with_path(path: PathBuf) -> Self {
        Store {
            path,
            crypto: Arc::new(Mutex::new(CryptoManager::new())),
            cache: Arc::new(Mutex::new(None)),
        }
    }

    fn load_vault(&self, master_password: &str) -> Result<HashMap<String, PasswordEntry>> {
        if !self.path.exists() {
            eprintln!("DEBUG: Vault file doesn't exists at {:?}", self.path);
            return Err(Error::NotInitialized);
        }

        let vault_json = fs::read_to_string(&self.path)?;
        let vault: EncryptedVault = serde_json::from_str(&vault_json)?;

        let mut crypto = self.crypto.lock().unwrap();
        let decrypted = crypto.open_vault(master_password, &vault)?;

        let data: HashMap<String, PasswordEntry> = serde_json::from_slice(&decrypted)?;

        Ok(data)
    }

    fn save_vault(&self, data: &HashMap<String, PasswordEntry>) -> Result<()> {
        let json_data = serde_json::to_vec(data)?;

        let crypto = self.crypto.lock().unwrap();
        if !crypto.is_unlocked() {
            return Err(Error::Encryption("Vault is locked".to_string()));
        }

        let (nonce, ciphertext) = crypto.encrypt(&json_data)?;

        let existing_vault: EncryptedVault = if self.path.exists() {
            let vault_json = fs::read_to_string(&self.path)?;
            serde_json::from_str(&vault_json)?
        } else {
            return Err(Error::Encryption("Cannot save - vault not initialized".to_string()));
        };

        let vault = EncryptedVault {
            salt: existing_vault.salt,
            argon2_params: existing_vault.argon2_params,
            nonce: encoding::encode(&nonce),
            ciphertext: encoding::encode(&ciphertext),
            version: 1
        };

        let vault_json = serde_json::to_string_pretty(&vault)?;
        fs::write(&self.path, vault_json)?;

        Ok(())
    }

    fn load_data(&self) -> Result<HashMap<String, PasswordEntry>> {
        if !self.path.exists() {
            return Err(Error::NotInitialized);
        }

        let contents = fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    fn save_data(&self, data: &HashMap<String, PasswordEntry>) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }
}


#[async_trait]
impl StoreRepository for Store {

    async fn init(&self, master_password: &str) -> Result<()> {
        if self.path.exists() {
            return Err(Error::AlreadyExists("Store".to_string()));
        }

        let data: HashMap<String, PasswordEntry> = HashMap::new();
        let json_data = serde_json::to_vec(&data)?;

        let mut crypto = self.crypto.lock().unwrap();
        let vault = crypto.create_vault(master_password, &json_data)?;

        let vault_json = serde_json::to_string_pretty(&vault)?;
        fs::write(&self.path, vault_json)?;

        let mut cache = self.cache.lock().unwrap();
        *cache = Some(data);


        Ok(())
    }

    async fn unlock(&self, master_password: &str) -> Result<()> {
        let data = self.load_vault(master_password)?;

        let mut cache = self.cache.lock().unwrap();
        *cache = Some(data);

        Ok(())
    }

    async fn lock(&self) -> Result<()> {
        let mut cache = self.cache.lock().unwrap();
        *cache = None;

        let mut crypto = self.crypto.lock().unwrap();
        crypto.lock();

        Ok(())
    }

    async fn is_locked(&self) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.is_none()
    }

    async fn exists(&self) -> bool {
        self.path.exists()
    }

    async fn add(&self, entry: PasswordEntry) -> Result<()> {
        let mut data = self.load_data()?;

        let key = entry.name.as_str().to_string();
        if data.contains_key(&key) {
             return Err(Error::AlreadyExists(key));
        }

        data.insert(key, entry);
        self.save_data(&data)?;
        Ok(())
    }

    async fn get(&self, name: &EntryName) -> Result<PasswordEntry> {
        let data = self.load_data()?;
        data.get(name.as_str())
            .cloned()
            .ok_or_else(|| Error::NotFound(name.as_str().to_string()))
    }

    async fn list(&self) -> Result<Vec<EntryName>> {
        let data = self.load_data()?;
        data.values()
            .map(|entry| Ok(entry.name.clone()))
            .collect()
    }

    async fn update(&self, entry: PasswordEntry) -> Result<()> {
        let mut data = self.load_data()?;
        data.insert(entry.name.as_str().to_string(), entry);
        self.save_data(&data)?;
        Ok(())
    }

    async fn remove(&self, name: &EntryName) -> Result<()> {
        let mut data = self.load_data()?;

        if data.remove(name.as_str()).is_none() {
            return Err(Error::NotFound(name.as_str().to_string()));
        }

        self.save_data(&data)?;
        Ok(())
    }

    async fn change_master_password(&self, current: &str, new: &str) -> Result<()> {
        // Verify current password and load data
        let data = self.load_vault(current)?;
        
        // Create new vault with new password
        let json_data = serde_json::to_vec(&data)?;
        let mut crypto = CryptoManager::new();
        let vault = crypto.create_vault(new, &json_data)?;
        
        // Save new vault
        let vault_json = serde_json::to_string_pretty(&vault)?;
        fs::write(&self.path, vault_json)?;
        
        // Update current crypto manager
        let mut current_crypto = self.crypto.lock().unwrap();
        *current_crypto = crypto;
        
        // Update cache
        let mut cache = self.cache.lock().unwrap();
        *cache = Some(data);
        
        Ok(())
    }
}



