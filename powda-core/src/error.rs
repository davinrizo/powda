
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotInitialized,
    AlreadyExists(String),
    NotFound(String),
    Io(std::io::Error),
    Serialization(serde_json::Error),
    Encryption(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        match self {
            Error::NotInitialized => write!(f, "Password store not initialized"),
            Error::AlreadyExists(name) => write!(f, "Entry '{}' already exists", name),
            Error::NotFound(name) => write!(f, "Entry '{}' not found", name),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Serialization(e) => write!(f, "Serialization error: {}", e),
            Error::Encryption(e) => write!(f, "Encryption error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialization(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

