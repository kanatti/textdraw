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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Idle,
    Selecting,
    Selected,
    Moving,
}
