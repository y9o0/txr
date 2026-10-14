pub use std::{
    io::{Write, stdout},
    process::exit,
};

pub use crossterm::{
    ExecutableCommand,
    cursor::{MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveUp},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, read},
    terminal::{Clear, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};

pub struct Cursor {
    pub current: u8,
    pub last_position: u8,
    pub row: u8,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            current: 0,
            last_position: 0,
            row: 0,
        }
    }
}
