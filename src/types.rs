//! Core types and enums used throughout the application.

use crate::element::Element;
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
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

/// Serializable format for saving/loading diagrams
#[derive(Serialize, Deserialize)]
pub struct DiagramFile {
    pub version: String,
    pub elements: Vec<Element>,
    pub next_id: usize,
}

impl DiagramFile {
    pub fn new(version: String, elements: Vec<Element>, next_id: usize) -> Self {
        Self {
            version,
            elements,
            next_id,
        }
    }
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
