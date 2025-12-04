//! Core types and enums used throughout the application.

use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

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
pub trait EventHandler: Sync {
    fn handle_key_event(&self, app: &mut App, key_event: &KeyEvent) -> EventResult {
        let _ = (app, key_event);
        EventResult::Ignored
    }

    fn handle_mouse_down(&self, app: &mut App, mouse_event: &MouseEvent) -> EventResult {
        let _ = (app, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_up(&self, app: &mut App, mouse_event: &MouseEvent) -> EventResult {
        let _ = (app, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_moved(&self, app: &mut App, mouse_event: &MouseEvent) -> EventResult {
        let _ = (app, mouse_event);
        EventResult::Ignored
    }

    fn handle_mouse_drag(&self, app: &mut App, mouse_event: &MouseEvent) -> EventResult {
        let _ = (app, mouse_event);
        EventResult::Ignored
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Canvas,
    Tools,
    Elements,
    Properties,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AppLayout {
    pub canvas: Option<Rect>,
    pub tools: Option<Rect>,
    pub elements: Option<Rect>,
    pub properties: Option<Rect>,
    pub statusbar: Option<Rect>,
}

/// Macro to define the Tool enum with associated names and keyboard shortcuts.
///
/// This generates:
/// - The Tool enum with all variants
/// - `all()` - returns all tools as a Vec
/// - `name()` - returns the display name for the tool
/// - `key()` - returns the keyboard shortcut for the tool
macro_rules! define_tools {
    ( $( $variant:ident => ($name:expr, $key:expr) ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Tool {
            $($variant),*
        }

        impl Tool {
            pub fn all() -> Vec<Tool> {
                vec![$(Tool::$variant),*]
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(Tool::$variant => $name),*
                }
            }

            pub fn key(&self) -> char {
                match self {
                    $(Tool::$variant => $key),*
                }
            }

            pub fn from_key(c: char) -> Option<Self> {
                match c {
                    $($key => Some(Tool::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

// Single place to define all tools with their names and keys
define_tools! {
    Select    => ("Select", 's'),
    Line      => ("Line", 'l'),
    Rectangle => ("Rectangle", 'r'),
    Arrow     => ("Arrow", 'a'),
    Text      => ("Text", 't'),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Idle,
    Selecting,
    Selected,
    Moving,
}
