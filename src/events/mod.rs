mod global;

use crate::app::App;
use crate::components::{CanvasComponent, ElementsPanel, HelpModal, PropertiesPanel, StatusBar, ToolsPanel};
use crate::types::{ActionType, EventHandler, EventResult};
use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEventKind};

use global::GlobalHandler;

/// Type alias for a slice of event handlers
type EventHandlers<'a> = &'a [&'a dyn EventHandler];

pub fn default_handlers() -> Vec<&'static dyn EventHandler> {
    vec![
        &HelpModal,
        &ToolsPanel,
        &ElementsPanel,
        &PropertiesPanel,
        &CanvasComponent,
        &StatusBar,
        &GlobalHandler,
    ]
}

macro_rules! dispatch_event {
    ($handlers:expr, $app:expr, $event:expr, $method:ident) => {{
        for handler in $handlers {
            match handler.$method($app, $event) {
                EventResult::Consumed => break,
                EventResult::Action(ActionType::Quit) => return Ok(true),
                EventResult::Ignored => continue,
            }
        }
    }};
}

pub fn handle_event(app: &mut App, event: Event, handlers: EventHandlers) -> Result<bool> {
    match event {
        Event::Key(key_event) => handle_key_event(app, key_event, handlers),
        Event::Mouse(mouse_event) => handle_mouse_event(app, mouse_event, handlers),
        _ => Ok(false),
    }
}

fn handle_key_event(app: &mut App, key_event: KeyEvent, handlers: EventHandlers) -> Result<bool> {
    dispatch_event!(handlers, app, &key_event, handle_key_event);
    Ok(false)
}

fn handle_mouse_event(app: &mut App, mouse_event: crossterm::event::MouseEvent, handlers: EventHandlers) -> Result<bool> {
    match mouse_event.kind {
        MouseEventKind::Down(_) => dispatch_event!(handlers, app, &mouse_event, handle_mouse_down),
        MouseEventKind::Up(_) => dispatch_event!(handlers, app, &mouse_event, handle_mouse_up),
        MouseEventKind::Moved => dispatch_event!(handlers, app, &mouse_event, handle_mouse_moved),
        MouseEventKind::Drag(_) => dispatch_event!(handlers, app, &mouse_event, handle_mouse_drag),
        MouseEventKind::ScrollDown
        | MouseEventKind::ScrollUp
        | MouseEventKind::ScrollLeft
        | MouseEventKind::ScrollRight => {
            dispatch_event!(handlers, app, &mouse_event, handle_mouse_scroll)
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests;