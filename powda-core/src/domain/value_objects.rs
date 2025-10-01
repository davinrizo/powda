use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryName(String);

impl EntryName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.is_empty() {
        return Err("Entry name cannot be empty".to_string());
        }

        if name.len() > 100 {
            return Err("Entry name too long(max 100 chars)".to_string());
        }

        Ok(EntryName(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);


impl Password {
    pub fn new(password: String) -> Result<Self, String> {
        if password.is_empty() {
            return Err("Password Cannot be empty".to_string());
        }
        Ok(Password(password))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
