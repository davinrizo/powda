use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedVault {
    pub salt: String,
    pub argon2_params: String,
    pub nonce: String,
    pub ciphertext: String,
    pub version: u32,
}