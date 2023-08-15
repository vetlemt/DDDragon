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
            app.world.camera_pitch -= 0.05;
        }
        KeyCode::Down => {
            app.world.camera_pitch += 0.05;
        }
        KeyCode::Left => {
            app.world.camera_yaw -= 0.05;
        }
        KeyCode::Right => {
            app.world.camera_yaw += 0.05;
        }

        KeyCode::Char('w') => {
            app.world.world_translation_z -= 0.05;
        }
        KeyCode::Char('a') => {
            app.world.world_translation_x += 0.05;
        }
        KeyCode::Char('s') => {
            app.world.world_translation_z += 0.05;
        }
        KeyCode::Char('d') => {
            app.world.world_translation_x -= 0.05;
        }
        _ => {}
    }
    Ok(())
}
