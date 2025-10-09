use thiserror::Error;

pub type Result<T> = std::result::Result<T, TuiError>;

#[derive(Error, Debug)]
pub enum TuiError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Core error: {0}")]
    Core(#[from] powda_core::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("No vault found. Please run 'powda init' first")]
    NoVault,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Locked,
    Main,
    AddingEntry,
    ViewingEntry(String),
    Search,
    Help,
    ConfirmDelete(String),
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    EntryName,
    EntryPassword,
    Search,
    MasterPassword
}

#[derive(Debug)]
pub struct AppContext {
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Success,
    Error,
    Info,
    Warning
}

impl MessageType {
    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            MessageType::Success => Color::Green,
            MessageType::Error => Color::Red,
            MessageType::Info => Color::Cyan,
            MessageType::Warning => COlor::Yellow,
        }
    }
}
