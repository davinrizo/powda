use powda_core::{Store, PasswordEntry, EntryName, Password, Result};
use powda_core::repository::StoreRepository;
use powda_core::error::Error;
use crate::ui;

pub struct PasswordHandler {
    store: Box<dyn StoreRepository>,
}

impl PasswordHandler {
    pub fn new() -> Self {
        Self {
            store: Box::new(Store::new()),
        }
    }

    pub async fn init(&self, force:bool) -> Result<()> {
    if self.store.exists().await && !force {
            println!("⚠️  Password vault already exists!");
            println!("Use 'powda init --force' to reinitialize");
            return Err(Error::AlreadyExists("Vault".to_string()));
        }

        if force && self.store.exists().await {
            println!("Force initializing will delete all existing passwords!");
            println!("Are you sure? (y/N)");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).map_err(Error::from)?;

            if input.trim().to_lowercase() != "y" {
                println!("Cancelled.");
                return Ok(())
            }

            let home = std::env::var("HOME").expect("HOME not set");

            let old_path = std::path::PathBuf::from(&home).join(".powda_store.json");
            let new_path = std::path::PathBuf::from(&home).join(".powda_vault.encrypted");

            if old_path.exists() {
                std::fs::remove_file(old_path).ok();
            }
            if new_path.exists() {
                std::fs::remove_file(new_path).ok();
            }
        }
        
        // Get master password
        let password = ui::prompt_password("Enter master password: ")?;
        let confirm = ui::prompt_password("Confirm master password: ")?;
        
        if password != confirm {
            return Err(Error::Encryption("Passwords don't match".to_string()));
        }
        
        // Check password strength
        if password.len() < 8 {
            return Err(Error::Encryption("Password must be at least 8 characters".to_string()));
        }
        
        self.store.init(&password).await?;
        println!("✅ Secure vault initialized!");
        println!("⚠️  Remember your master password - it cannot be recovered!");
        Ok(())
    }

    pub async fn unlock(&self) -> Result<()> {
        let password = ui::prompt_password("Enter master password: ")?;
        self.store.unlock(&password).await?;
        println!("Vault unlocked!");
        Ok(())
    }

    pub async fn add(&self, name: String) -> Result<()> {

        if self.store.is_locked().await {
            println!("🔒 Vault is locked. Unlocking...");
            self.unlock().await?;
        }

        let entry_name = EntryName::new(name.clone())
            .map_err(|e| powda_core::Error::Encryption(e))?;


        let password = ui::prompt_password("Enter Password: ")?;
        let password = Password::new(password)
            .map_err(|e| powda_core::Error::Encryption(e))?;

        let entry = PasswordEntry::new(entry_name, password);
        self.store.add(entry).await?;

        println!("Password for '{}' added!", name);
        Ok(())
    }

    pub async fn get(self, name: String) -> Result<()> {
        let entry_name = EntryName::new(name)
            .map_err(|e| powda_core::Error::Encryption(e))?;

        let entry = self.store.get(&entry_name).await?;
        println!("Password: {}", entry.password.as_str());
        Ok(())
    }

    pub async fn list(&self) -> Result<()> {

        let entries = self.store.list().await?;
        if entries.is_empty() {
            println!("No passwords stored.");
        } else {
            println!("Stored password: ");
            for entry_name in entries {
                println!(" * {}", entry_name.as_str());
            }
        }
        Ok(())
    }

    pub async fn remove(&self, name: String) -> Result<()> {
        let entry_name = EntryName::new(name.clone())
            .map_err(|e| powda_core::Error::Encryption(e))?;

        println!("Are you sure want to remove '{}'? (y/N", name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .map_err(powda_core::Error::from)?;

        if input.trim().to_lowercase() == "y" {
            self.store.remove(&entry_name).await?;
            println!("Password for '{}' removed!", name);
        } else {
            println!("Cancelled!");
        }

        Ok(())
    }
    pub async fn lock(&self) -> Result<()> {
    self.store.lock().await?;
    println!("Vault locked!");
    Ok(())
}

    pub async fn change_master(&self) -> Result<()> {
        let current = rpassword::prompt_password("Enter current master password: ")?;
        let new = rpassword::prompt_password("Enter new master password: ")?;
        let confirm = rpassword::prompt_password("Confirm new master password: ")?;
    
        if new != confirm {
            return Err(Error::Encryption("Passwords don't match".to_string()));
        }
    
        self.store.change_master_password(&current, &new).await?;
        println!("Master password changed successfully!");
        Ok(())
    }
}
