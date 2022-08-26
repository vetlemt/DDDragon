

use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use dddragon::app::{App, AppResult};
use dddragon::event::{Event, EventHandler};
use dddragon::handler::handle_key_events;
use dddragon::tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Default => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}