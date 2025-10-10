use crate::tui::{
    events::EventHandler,
    types::{AppContext, AppState, InputMode, MessageType, Result, TuiError},
    ui::UI,
};
use powda_core::{Store, StoreRepository, PasswordEntry, EntryName, Password};
use ratatui::{backend::Backend, Terminal};
use std::time::Duration;
use tui_input::Input;

pub struct App {
    store: Box<dyn StoreRepository>,
    context: AppContext,
    entries: Vec<String>,
    filtered_entries: Vec<String>,
    selected_index: usize,
    input: Input,
    password_input: String,
    search_query: String,
    show_password: bool,
    event_handler: EventHandler,
    ui:UI
}

impl App {
    pub async fn new() -> Result<Self> {
        let store = Box::new(Store::new());

        if !store.exists().await {
            return Err(TuiError::NoVault);
        }

        Ok(Self {
            store,
            context: AppContext {
                state: AppState:Locked,
                input_mode: InputMode::MasterPassword,
                message: None,
            },
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            selected_index: 0,
            input: Input::default(),
            password_input: String::new(),
            search_query: String::new(),
            show_password: false,
            event_handler: EventHandler::new(),
            ui: UI::new(),
        })
    }

    pub async fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop{
            terminal.draw(|f| self.ui.render(f, &self))?;

            if let Some(event) = self.event_handler.next(Duration::from_millis(100))? {
                if self.handle_event(event).await? {
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_event(&mut self, event: crossterm::event::Event) -> Result<bool> {
        use crossterm::event::{Event, KeyCode, KeyEventKind};

        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            // Delegate to specific handlers
            match self.context.input_mode {
                // InputMode::Normal 
                // InputMode::Normal 
                // InputMode::Normal 
                // InputMode::Normal 
                // InputMode::Normal 
            }
        }
        Ok(false)
    }

    async fn handle_normal_mode(&mut self, key: KeyCode) -> Result<bool> {
        use crossterm::event::KeyCode;

        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
            KeyCode::Char('a') => self.start_add_entry(),
            KeyCode::Char('/') => self.start_search(),
            KeyCode::Char('?') => self.show_help(),
            KeyCode::Enter => self.view_selected_entry().await?,
            KeyCode::Char('d') | KeyCode::Delete => self.confirm_delete_selected(),

            KeyCode::Up | KeyCode::Char('k') => self.move_selection_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_selection_down(),
            KeyCode::Esc => self.return_to_main(),
            _ => {}
        }

        Ok(false)
    }

    async fn unlock(&mut self, password: &str) -> Result<()> {
        match self.store.unlock(password).await {
            Ok(_) => {
                self.load_entries().await?;
                self.context.state = AppState::Main;
                self.context.input_mode = InputMode::Normal;
                self.set_message("Vault unlocked!", MessageType::Success);
                Ok(())
            }
            Err(e) => {
                self.set_message(&format!("Failed to unlock: {}", e), MessageType::Error);
                Err(TuiError::Core(e))
            }
        }
    }

    async fn load_entries(&mut self) -> Result<()> {
        match self.store.list().await {
            Ok(entries) => {
                self.entries = entries.into_iter()
                    .map(|e| e.as_str().to_string())
                    .collect();
                self.entries.sort();
                self.finltered_entries = self.entries.clone();
                Ok(())
            }
            Err(e) => Err(TuiError::Core(e))
        }
    }

    fn set_message(&mut self, msg: &str, msg_type: MessageType) {
        self.context.message = Some((msg.to_string(), msg_type));
    }
}


