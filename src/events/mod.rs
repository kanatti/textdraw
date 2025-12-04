mod mouse;

use crate::app::App;
use crate::components::{CanvasComponent, ElementsPanel, PropertiesPanel, StatusBar, ToolsPanel};
use crate::types::{EventHandler, EventResult, Panel, Tool};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};

static EVENT_HANDLERS: &[&dyn EventHandler] = &[
    &ToolsPanel,
    &ElementsPanel,
    &PropertiesPanel,
    &CanvasComponent,
    &StatusBar,
];

pub fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    match event {
        Event::Key(key_event) => handle_key_event(app, key_event),
        Event::Mouse(mouse_event) => handle_mouse_event(app, mouse_event),
        _ => Ok(false),
    }
}

fn handle_key_event(app: &mut App, key_event: KeyEvent) -> Result<bool> {
    // Give components first chance to handle key events
    for handler in EVENT_HANDLERS {
        if handler.handle_key_event(app, &key_event) == EventResult::Consumed {
            return Ok(false);
        }
    }

    // Fall through to global key handling
    match key_event.code {
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

fn handle_mouse_event(app: &mut App, mouse_event: crossterm::event::MouseEvent) -> Result<bool> {
    // Give components first chance to handle mouse events
    for handler in EVENT_HANDLERS {
        if handler.handle_mouse_event(app, &mouse_event) == EventResult::Consumed {
            return Ok(false);
        }
    }

    // Fall through to existing handler
    mouse::handle_mouse_event(app, mouse_event.kind, mouse_event.column, mouse_event.row);
    Ok(false)
}