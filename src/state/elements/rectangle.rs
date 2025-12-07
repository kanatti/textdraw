use crate::types::{Bounds, Coord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectangleElement {
    pub id: usize,
    pub name: String,
    pub start: Coord,
    pub width: u16,
    pub height: u16,
    pub bounds: Bounds,
}

impl RectangleElement {
    pub fn new(id: usize, start: Coord, width: u16, height: u16) -> Self {
        let name = format!("Rectangle {}", id + 1);
        let bounds = Bounds {
            min: start,
            max: Coord {
                x: start.x.saturating_add(width),
                y: start.y.saturating_add(height),
            },
        };
        Self {
            id,
            name,
            start,
            width,
            height,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.start.translate(dx, dy);
        self.bounds.translate(dx, dy);
    }
}
