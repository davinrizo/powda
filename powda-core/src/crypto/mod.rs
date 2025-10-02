use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{Error, Result};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey {
    key: Vec<u8>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub salt: String,
    pub argon2_params: String,
    pub nonce: String,
    pub ciphertext: String,
    pub version: u32,
}

pub struct CryptoManager {
    master_key: Option<MasterKey>,
}

impl CryptoManager {
    pub fn new() -> {
        Self {master_key: None}
    }

    pub fn derive_master_key(&mut self, password: &str, salt: Option<&str>) -> Result<String> {
        // use provided salt or generate a new one
        let salt_string = match salt {
            Some(s) => SaltString::from_b64(s)
                .map_err(|e| Error::Encryption(format!("Invalid salt: {}",e)))?,
            None => SaltString::generate(&mut OsRng),
        };

        // Define Argon2 
        let argon2 = Argon2::default();

        // Hash Password
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| Error::Encryption(format!("Failed to hash password: {}", e)))?;

        let hash_bytes = password_hash.hash.unwrap();
        let mut key = vec![0u8; 32];

        let mut hasher = Sha256::new();
        hasher.update(hash_bytes.as_bytes());
        let result = hasher.finalize();
        key.copy_from_slice(&result[..32]);

        self.master_key = Some(MasterKey {key: key.clone()});

        Ok(salt_string.to_string())
    }

    pub fn verify_password(&mut self, password: &str, vault: &EncryptedVault) -> Result<()> {
        let parsed_hash = PasswordHash::new(&vault.argon2_params)
            .map_err(|e| Error::Encryption(format!("Invalid stored Hash: {}", e)))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::Encryption("Invalid master password".to_string()))?;

        Self.derive_master_key(password, Some(&vault.salt))?;

        Ok(())
    }
}
