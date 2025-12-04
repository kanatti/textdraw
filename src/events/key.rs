use crate::app::App;
use crate::types::{Panel, Tool};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) -> Result<bool> {
    // If text tool is active and in drawing mode, handle text input
    if app.is_text_input_mode() {
        return handle_text_input(app, key_event.code);
    }

    // Try panel-specific handlers first
    if app.active_panel == Panel::Tools {
        if handle_tools_panel_keys(app, key_event.code)? {
            return Ok(false);
        }
    }

    // Fall back to global key handling
    handle_keycode(app, key_event.code)
}

fn handle_keycode(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        // Panel shortcuts
        KeyCode::Char(c @ '0'..='3') => {
            let panel = match c {
                '0' => Panel::Canvas,
                '1' => Panel::Tools,
                '2' => Panel::Elements,
                '3' => Panel::Properties,
                _ => unreachable!("Unhandled panel switch"),
            };
            app.switch_panel(panel);
            Ok(false)
        }
        // Tool shortcuts
        KeyCode::Esc => {
            app.select_tool(Tool::Select);
            Ok(false)
        }
        // Tool selection - automatically handles all tools defined in types.rs
        KeyCode::Char(c) => {
            if let Some(tool) = Tool::from_key(c) {
                app.select_tool(tool);
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

/// Handle keys specific to Tools panel. Returns true if key was handled.
fn handle_tools_panel_keys(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.select_prev_tool();
            Ok(true)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.select_next_tool();
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Handle text input when text tool is active
fn handle_text_input(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char(c) => {
            app.add_text_char(c);
            Ok(false)
        }
        KeyCode::Backspace => {
            app.text_backspace();
            Ok(false)
        }
        KeyCode::Enter | KeyCode::Esc => {
            // Commit or cancel text
            if key_code == KeyCode::Enter {
                app.finish_text_input();
            } else {
                app.cancel_drawing();
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}
