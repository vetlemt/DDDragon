use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on ESC
        KeyCode::Esc => {
            app.running = false;
        }
        // exit application on Ctrl-D
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        KeyCode::Up => {
            app.n -= 0.05;
        }
        KeyCode::Down => {
            app.n += 0.05;
        }
        KeyCode::Left => {
            app.m -= 0.05;
        }
        KeyCode::Right => {
            app.m += 0.05;
        }
        _ => {}
    }
    Ok(())
}
