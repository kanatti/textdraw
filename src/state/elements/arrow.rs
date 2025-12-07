use super::segment;
use super::Segment;
use crate::types::Bounds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowElement {
    pub id: usize,
    pub name: String,
    pub segments: Vec<Segment>,
    pub bounds: Bounds,
}

impl ArrowElement {
    pub fn new(id: usize, segments: Vec<Segment>) -> Self {
        let name = format!("Arrow {}", id + 1);
        let bounds = segment::calculate_bounds(&segments);
        Self {
            id,
            name,
            segments,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        for segment in &mut self.segments {
            segment.translate(dx, dy);
        }
        self.bounds.translate(dx, dy);
    }
}
