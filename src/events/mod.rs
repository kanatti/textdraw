mod global;
mod types;

pub use global::GlobalHandler;
pub use types::{KeyEvent, MouseEvent, MouseEventKind};

use crate::state::AppState;
use anyhow::Result;
use crossterm::event::Event;

/// Actions that can be triggered by event handlers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    /// Quit the application
    Quit,
    /// Tool finished drawing an element
    FinishedDrawing,
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
    type State;

    fn handle_key_event(&mut self, state: &mut Self::State, key_event: &KeyEvent) -> EventResult {
        let _ = (state, key_event);
        EventResult::Ignored
    }

    fn handle_mouse_down(
        &mut self,
        state: &mut Self::State,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_up(
        &mut self,
        state: &mut Self::State,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_moved(
        &mut self,
        state: &mut Self::State,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_drag(
        &mut self,
        state: &mut Self::State,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_scroll(
        &mut self,
        state: &mut Self::State,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let _ = (state, mouse_event);
        EventResult::Ignored
    }
}

/// Type alias for a slice of mutable event handlers
type EventHandlers<'a> = &'a mut [&'a mut dyn EventHandler<State = AppState>];

macro_rules! dispatch_event {
    ($handlers:expr, $state:expr, $event:expr, $method:ident) => {{
        for handler in $handlers.iter_mut() {
            match handler.$method($state, $event) {
                EventResult::Consumed => break,
                EventResult::Action(ActionType::Quit) => return Ok(true),
                EventResult::Action(_) => break, // Other actions are consumed by handlers
                EventResult::Ignored => continue,
            }
        }
    }};
}

pub fn handle_event(event: Event, handlers: EventHandlers, state: &mut AppState) -> Result<bool> {
    match event {
        Event::Key(key_event) => {
            // Convert crossterm KeyEvent to our KeyEvent
            let our_event = KeyEvent::from(key_event);
            dispatch_event!(handlers, state, &our_event, handle_key_event);
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
    // Convert crossterm MouseEvent to our MouseEvent
    let our_event = MouseEvent::from(mouse_event);

    match our_event.kind {
        MouseEventKind::Down(_) => {
            dispatch_event!(handlers, state, &our_event, handle_mouse_down)
        }
        MouseEventKind::Up(_) => dispatch_event!(handlers, state, &our_event, handle_mouse_up),
        MouseEventKind::Moved => dispatch_event!(handlers, state, &our_event, handle_mouse_moved),
        MouseEventKind::Drag(_) => {
            dispatch_event!(handlers, state, &our_event, handle_mouse_drag)
        }
        MouseEventKind::ScrollDown
        | MouseEventKind::ScrollUp
        | MouseEventKind::ScrollLeft
        | MouseEventKind::ScrollRight => {
            dispatch_event!(handlers, state, &our_event, handle_mouse_scroll)
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests;
