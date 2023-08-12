use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on ESC
        KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Up => {
            app.a -= 0.05;
        }
        KeyCode::Down => {
            app.a += 0.05;
        }
        KeyCode::Left => {
            app.b -= 0.05;
        }
        KeyCode::Right => {
            app.b += 0.05;
        }

        KeyCode::Char('w') => {
            app.posz -= 0.05;
        }
        KeyCode::Char('a') => {
            app.posx += 0.05;
        }
        KeyCode::Char('s') => {
            app.posz += 0.05;
        }
        KeyCode::Char('d') => {
            app.posx -= 0.05;
        }
        _ => {}
    }
    Ok(())
}
