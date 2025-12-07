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
    /// Create a new mouse event
    pub fn new(column: u16, row: u16, kind: MouseEventKind, modifiers: KeyModifiers) -> Self {
        Self {
            column,
            row,
            kind,
            modifiers,
        }
    }

    /// Create a new event with offset coordinates
    pub fn offset(&self, dx: i16, dy: i16) -> Self {
        Self {
            column: self.column.saturating_add_signed(dx),
            row: self.row.saturating_add_signed(dy),
            ..*self
        }
    }

    /// Create a new event with specific coordinates
    pub fn with_coords(&self, column: u16, row: u16) -> Self {
        Self {
            column,
            row,
            ..*self
        }
    }

    /// Get the mouse button from the event kind
    pub fn button(&self) -> MouseButton {
        match self.kind {
            MouseEventKind::Down(btn) | MouseEventKind::Up(btn) | MouseEventKind::Drag(btn) => btn,
            _ => MouseButton::Left, // Default for Moved and Scroll events
        }
    }

    /// Get coordinates as tuple
    pub fn coords(&self) -> (u16, u16) {
        (self.column, self.row)
    }

    /// Get coordinates as signed integers
    pub fn coords_i32(&self) -> (i32, i32) {
        (self.column as i32, self.row as i32)
    }

    /// Check if a modifier key is pressed
    pub fn has_modifier(&self, modifier: KeyModifiers) -> bool {
        self.modifiers.contains(modifier)
    }

    /// Check if Shift is pressed
    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    /// Check if Ctrl is pressed
    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if Alt is pressed
    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
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

impl KeyEvent {
    /// Create a new key event
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    /// Check if a modifier key is pressed
    pub fn has_modifier(&self, modifier: KeyModifiers) -> bool {
        self.modifiers.contains(modifier)
    }

    /// Check if Shift is pressed
    pub fn is_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }

    /// Check if Ctrl is pressed
    pub fn is_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if Alt is pressed
    pub fn is_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    /// Check if only Ctrl is pressed (no other modifiers)
    pub fn is_ctrl_only(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL
    }

    /// Check if only Shift is pressed (no other modifiers)
    pub fn is_shift_only(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT
    }

    /// Check if no modifiers are pressed
    pub fn is_plain(&self) -> bool {
        self.modifiers.is_empty()
    }
}

impl From<crossterm::event::KeyEvent> for KeyEvent {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}
