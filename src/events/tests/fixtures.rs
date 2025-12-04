use crate::app::App;
use crate::types::{ActionType, EventHandler, EventResult};
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
impl EventHandler for IgnoreHandler {}

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
    fn handle_key_event(&self, _app: &mut App, _key_event: &KeyEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_down(&self, _app: &mut App, _mouse_event: &MouseEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_up(&self, _app: &mut App, _mouse_event: &MouseEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_moved(&self, _app: &mut App, _mouse_event: &MouseEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }

    fn handle_mouse_drag(&self, _app: &mut App, _mouse_event: &MouseEvent) -> EventResult {
        self.called.mark_called();
        EventResult::Consumed
    }
}

// Dummy handler that returns quit action for 'q' key
pub struct QuitHandler;
impl EventHandler for QuitHandler {
    fn handle_key_event(&self, _app: &mut App, key_event: &KeyEvent) -> EventResult {
        match key_event.code {
            KeyCode::Char('q') => EventResult::Action(ActionType::Quit),
            _ => EventResult::Ignored,
        }
    }
}

// Helper functions to create test events
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
