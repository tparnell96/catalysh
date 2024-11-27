// src/handlers/clear.rs

use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};

pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).expect("Failed to clear screen");
}
