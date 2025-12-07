mod global;

pub use global::GlobalHandler;

use crate::state::AppState;
use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent, MouseEventKind};

/// Actions that can be triggered by event handlers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    /// Quit the application
    Quit,
}

/// Result of handling an event - consumed or ignored for event propagation control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Event was handled, stop propagation
    Consumed,
    /// Event was not handled, continue to next component
    Ignored,
    /// Event triggers an application-level action
    Action(ActionType),
}

/// Event handler trait for components
pub trait EventHandler {
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        let _ = (state, key_event);
        EventResult::Ignored
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_up(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_moved(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_drag(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_scroll(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }
}

/// Type alias for a slice of mutable event handlers
type EventHandlers<'a> = &'a mut [&'a mut dyn EventHandler];

macro_rules! dispatch_event {
    ($handlers:expr, $state:expr, $event:expr, $method:ident) => {{
        for handler in $handlers.iter_mut() {
            match handler.$method($state, $event) {
                EventResult::Consumed => break,
                EventResult::Action(ActionType::Quit) => return Ok(true),
                EventResult::Ignored => continue,
            }
        }
    }};
}

pub fn handle_event(event: Event, handlers: EventHandlers, state: &mut AppState) -> Result<bool> {
    match event {
        Event::Key(key_event) => {
            dispatch_event!(handlers, state, &key_event, handle_key_event);
            Ok(false)
        }
        Event::Mouse(mouse_event) => handle_mouse_event(mouse_event, handlers, state),
        _ => Ok(false),
    }
}

fn handle_mouse_event(
    mouse_event: crossterm::event::MouseEvent,
    handlers: EventHandlers,
    state: &mut AppState,
) -> Result<bool> {
    match mouse_event.kind {
        MouseEventKind::Down(_) => {
            dispatch_event!(handlers, state, &mouse_event, handle_mouse_down)
        }
        MouseEventKind::Up(_) => dispatch_event!(handlers, state, &mouse_event, handle_mouse_up),
        MouseEventKind::Moved => dispatch_event!(handlers, state, &mouse_event, handle_mouse_moved),
        MouseEventKind::Drag(_) => {
            dispatch_event!(handlers, state, &mouse_event, handle_mouse_drag)
        }
        MouseEventKind::ScrollDown
        | MouseEventKind::ScrollUp
        | MouseEventKind::ScrollLeft
        | MouseEventKind::ScrollRight => {
            dispatch_event!(handlers, state, &mouse_event, handle_mouse_scroll)
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests;
