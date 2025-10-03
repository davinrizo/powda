use base64::{engine::general_purpose::STANDARD, Engine};

    pub fn encode(data: &[u8]) -> String {
        STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError>{
        STANDARD.decode(data)
    }