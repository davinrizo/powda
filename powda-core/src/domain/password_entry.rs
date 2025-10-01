
use super::value_objects::{EntryName, Password};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: EntryName,
    pub password: Password,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub metadata: EntryMetadata
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntryMetadata {
    pub url: Option<String>,
    pub username: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String> 
}

impl PasswordEntry {
    pub fn new(name: EntryName, password: Password) -> Self {
        let now = SystemTime::now();
        Self {
            name,
            password,
            created_at: now,
            modified_at: now,
            metadata: EntryMetadata::default(),
        }
    }

    pub fn update_password(&mut self, password: Password) {
        self.password = password;
        self.modified_at = SystemTime::now();
    }
}
