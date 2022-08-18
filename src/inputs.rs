use crossterm::event::{self, Event, KeyCode};
use std::io;

use crate::state::App;

pub enum InputMode {
    Normal,
    Editing,
}

pub fn input_handler(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    // TODO:
                    // app.pods.push(app.input.drain(..).collect());
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
        }
    };
    Ok(false)
}
