use crate::events::{ActionType, EventHandler, EventResult};
use crate::events::{KeyEvent as OurKeyEvent, MouseEvent as OurMouseEvent};
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use std::sync::atomic::{AtomicBool, Ordering};

/// Trait for tracking whether a handler was called in tests
pub trait CallTracker: Sync {
    fn mark_called(&self);
    fn was_called(&self) -> bool;
}

impl CallTracker for AtomicBool {
    fn mark_called(&self) {
        self.store(true, Ordering::Relaxed);
    }

    fn was_called(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}

// Dummy handler that ignores all events (uses default trait implementations)
pub struct IgnoreHandler;
impl EventHandler for IgnoreHandler {
    type State = AppState;
}

// Dummy handler that tracks if it was called and consumes all events
pub struct ConsumeHandler {
    called: AtomicBool,
}

impl ConsumeHandler {
    pub fn new() -> Self {
        Self {
            called: AtomicBool::new(false),
        }
    }

    pub fn was_called(&self) -> bool {
        self.called.was_called()
    }
}

impl EventHandler for ConsumeHandler {
    type State = AppState;

    fn handle_key_event(&mut self, _state: &mut AppState, _key_event: &OurKeyEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_down(
        &mut self,
        _state: &mut AppState,
        _mouse_event: &OurMouseEvent,
    ) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_up(
        &mut self,
        _state: &mut AppState,
        _mouse_event: &OurMouseEvent,
    ) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_moved(
        &mut self,
        _state: &mut AppState,
        _mouse_event: &OurMouseEvent,
    ) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_drag(
        &mut self,
        _state: &mut AppState,
        _mouse_event: &OurMouseEvent,
    ) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }
}

// Dummy handler that returns quit action for 'q' key
pub struct QuitHandler;
impl EventHandler for QuitHandler {
    type State = AppState;

    fn handle_key_event(&mut self, _state: &mut AppState, key_event: &OurKeyEvent) -> EventResult {
        match key_event.code {
            KeyCode::Char('q') => EventResult::Action(ActionType::Quit),
            _ => EventResult::Ignored,
        }
    }
}

// Helper functions to create test events (returns crossterm types for use with handle_event)
pub fn key_event(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
}

pub fn mouse_down() -> MouseEvent {
    MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 0,
        row: 0,
        modifiers: KeyModifiers::NONE,
    }
}
