//! Core types and enums used throughout the application.

use ratatui::layout::Rect;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Idle,
    Selecting,
    Selected,
    Moving,
}
