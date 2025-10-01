use powda_core::{Store, PasswordEntry, EntryName, Password, Result};
use powda_core::repository::StoreRepository;
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

    pub async fn init(&self) -> Result<()> {
        self.store.init().await?;
        println!("Password store initialized!");
        Ok(())
    }

    pub async fn add(&self, name: String) -> Result<()> {
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
}
