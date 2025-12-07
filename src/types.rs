//! Core types and enums used throughout the application.

use serde::{Deserialize, Serialize};

/// A render point with position and character
pub type RenderPoint = (i32, i32, char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

impl Coord {
    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add_signed(dx);
        self.y = self.y.saturating_add_signed(dy);
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bounds {
    pub min: Coord,
    pub max: Coord,
}

impl Bounds {
    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.min.translate(dx, dy);
        self.max.translate(dx, dy);
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Canvas,
    Tools,
    Elements,
    Properties,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Idle,
    Selecting,
    Selected,
    Moving,
}
