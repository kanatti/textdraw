use crossterm::event::{KeyCode, KeyModifiers, MouseButton};

/// Mouse event kind - what type of mouse event occurred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

/// Our own mouse event type with helper methods for coordinate transformations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub column: u16,
    pub row: u16,
    pub kind: MouseEventKind,
    pub modifiers: KeyModifiers,
}

impl MouseEvent {
    /// Create a new event with specific coordinates
    pub fn with_coords(&self, column: u16, row: u16) -> Self {
        Self {
            column,
            row,
            ..*self
        }
    }

    /// Check if Shift is pressed
    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }
}

impl From<crossterm::event::MouseEvent> for MouseEvent {
    fn from(event: crossterm::event::MouseEvent) -> Self {
        // Convert crossterm's MouseEventKind to our MouseEventKind
        let kind = match event.kind {
            crossterm::event::MouseEventKind::Down(btn) => MouseEventKind::Down(btn),
            crossterm::event::MouseEventKind::Up(btn) => MouseEventKind::Up(btn),
            crossterm::event::MouseEventKind::Drag(btn) => MouseEventKind::Drag(btn),
            crossterm::event::MouseEventKind::Moved => MouseEventKind::Moved,
            crossterm::event::MouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            crossterm::event::MouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
            crossterm::event::MouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            crossterm::event::MouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
        };

        Self {
            column: event.column,
            row: event.row,
            kind,
            modifiers: event.modifiers,
        }
    }
}

/// Our own keyboard event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}
