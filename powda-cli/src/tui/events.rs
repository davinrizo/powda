use crossterm::event::{self, Event};
use std::time::Duration;
use crate::tui:types::Result;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next(&self, timeout:Duration) -> Result<Option<Event>> {
        if event::poll(timeout)? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}
