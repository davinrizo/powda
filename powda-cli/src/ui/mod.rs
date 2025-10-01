
use rpassword;
use powda_core::{Result, Error};

pub fn prompt_password(prompt: &str) -> Result<String> {
    rpassword::prompt_password(prompt)
            .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
}
