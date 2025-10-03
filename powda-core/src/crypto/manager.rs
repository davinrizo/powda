use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2
};
use rand::rngs::OsRng;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
// use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
// use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{Error, Result};

use super::master_key::MasterKey;
use super::vault::EncryptedVault;
use super::encoding;


pub struct CryptoManager {
    master_key: Option<MasterKey>,
}

impl CryptoManager {

    pub fn new() -> Self{
        Self {master_key: None}
    }

    pub fn derive_master_key(&mut self, password: &str, salt: Option<&str>) -> Result<String> {
        // use provided salt or generate a new one
        let salt_string = match salt {
            Some(s) => SaltString::from_b64(s)
                .map_err(|e| Error::Encryption(format!("Invalid salt: {}",e)))?,
            None => SaltString::generate(OsRng),
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

        self.derive_master_key(password, Some(&vault.salt))?;

        Ok(())
    }


    pub fn encrypt(&self, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let master_key = self.master_key.as_ref()
            .ok_or_else(|| Error::Encryption("No master key set".to_string()))?;

        let cipher = ChaCha20Poly1305::new_from_slice(&master_key.key)
            .map_err(|e| Error::Encryption(format!("Failed to create cipher: {}", e)))?;

        let nonce = ChaCha20Poly1305::generate_nonce()
            .map_err(|e| Error::Encryption(format!("Failed to generate nonce: {}", e)))?;

        let ciphertext = cipher.encrypt(&nonce, plaintext)
            .map_err(|e| Error::Encryption(format!("Encryption failed: {}", e)))?;
        
        Ok((nonce.to_vec(), ciphertext))
    }

    pub fn decrypt(&self, nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let master_key = self.master_key.as_ref()
            .ok_or_else(|| Error::Encryption("No master key set".to_string()))?;

        let cipher = ChaCha20Poly1305::new_from_slice(&master_key.key)
            .map_err(|e| Error::Encryption(format!("Failed to create cipher: {}", e)))?;

        let nonce = Nonce::try_from(nonce)
            .map_err(|_| Error::Encryption("Invalid nonce length".to_string()))?;

        let plaintext = cipher.decrypt(&nonce, ciphertext)
            .map_err(|_| Error::Encryption("Decryption failed - wrong password?".to_string()))?;

        Ok(plaintext)
    }

    pub fn create_vault(&mut self, password: &str, data: &[u8]) -> Result<EncryptedVault> {
        let salt = self.derive_master_key(password, None)?;

        let argon2 = Argon2::default();
        let salt_obj = SaltString::from_b64(&salt)
            .map_err(|e| Error::Encryption(format!("Invalid salt: {}", e)))?;

        let password_hash = argon2.hash_password(password.as_bytes(), &salt_obj)
            .map_err(|e| Error::Encryption(format!("Failed to hash password: {}", e)))?;

        let (nonce, ciphertext) = self.encrypt(data)?;

        Ok(EncryptedVault {
            salt,
            argon2_params: password_hash.to_string(),
            nonce: encoding::encode(&nonce),
            ciphertext: encoding::encode(&ciphertext),
            version: 1
        })
    }

    pub fn open_vault(&mut self, password: &str, vault: &EncryptedVault) -> Result<Vec<u8>> {
        self.verify_password(password, vault)?;

        let nonce = encoding::decode(&vault.nonce)
            .map_err(|e| Error::Encryption(format!("Invalid nonce: {}", e )))?;

        let ciphertext = encoding::decode(&vault.ciphertext)
            .map_err(|e| Error::Encryption(format!("Invalid ciphertext: {}", e)))?;

        self.decrypt(&nonce, &ciphertext)
    }

    pub fn is_unlocked(&self) -> bool {
        self.master_key.is_some()
    }

    pub fn lock(&mut self){
        self.master_key = None;
    }
}



