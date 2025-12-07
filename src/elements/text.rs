use crate::types::{Bounds, Coord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub id: usize,
    pub name: String,
    pub position: Coord,
    pub text: String,
    pub bounds: Bounds,
}

impl TextElement {
    pub fn new(id: usize, position: Coord, text: String) -> Self {
        let name = format!("Text {}", id + 1);
        let width = text.len() as u16;
        let bounds = Bounds {
            min: position,
            max: Coord {
                x: position.x.saturating_add(width),
                y: position.y,
            },
        };
        Self {
            id,
            name,
            position,
            text,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.position.translate(dx, dy);
        self.bounds.translate(dx, dy);
    }
}
