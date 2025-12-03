mod key;
mod mouse;

use crate::app::App;
use anyhow::Result;
use crossterm::event::Event;

pub fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    match event {
        Event::Key(event) => key::handle_key_event(app, event),
        Event::Mouse(mouse) => {
            mouse::handle_mouse_event(app, mouse.kind, mouse.column, mouse.row);
            Ok(false)
        }
        _ => Ok(false),
    }
}

