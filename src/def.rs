pub use std::{
    io::{Write, stdout},
    process::exit,
};

pub use crossterm::{
    ExecutableCommand,
    cursor::{MoveLeft, MoveRight, MoveTo, MoveToColumn},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, read},
    terminal::{Clear, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: u8,
    pub coulmen: u8,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, coulmen: 0 }
    }
}
