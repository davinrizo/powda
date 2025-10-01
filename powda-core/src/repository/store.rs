use crate::domain::{PasswordEntry, EntryName};
use crate::error::{Error, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

#[async_trait]
pub trait StoreRepository: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn exists(&self) -> bool;
    async fn add(&self, entry: PasswordEntry) -> Result<()>;
    async fn get(&self, name: &EntryName) -> Result<PasswordEntry>;
    async fn list(&self) -> Result<Vec<EntryName>>;
    async fn update(&self, entry: PasswordEntry) -> Result<()>;
    async fn remove(&self, name: &EntryName) -> Result<()>;
}

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Self {
        let home = std::env::var("HOME").expect("HOME not set");
        let path = PathBuf::from(home).join(".powda_store.json");
        Store {path}
    }

    pub fn with_path(path: PathBuf) -> Self {
        Store {path}
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
    async fn init(&self) -> Result<()> {
        if self.path.exists() {
            return Err(Error::AlreadyExists("Store".to_string()));
        }

        let data: HashMap<String, PasswordEntry> = HashMap::new();
        self.save_data(&data)?;
        Ok(())
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
}



