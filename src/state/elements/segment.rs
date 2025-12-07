use crate::types::{Bounds, Coord, Direction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start: Coord,
    pub length: u16,
    pub direction: Direction,
}

impl Segment {
    /// Create a segment from start and end coordinates
    /// Automatically determines direction and length
    pub fn from_coords(start: Coord, end: Coord) -> Self {
        let dx = (end.x as i32) - (start.x as i32);
        let dy = (end.y as i32) - (start.y as i32);

        let direction = if dx.abs() > dy.abs() {
            // Horizontal line
            if dx > 0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            // Vertical line
            if dy > 0 {
                Direction::Down
            } else {
                Direction::Up
            }
        };

        let length = match direction {
            Direction::Right | Direction::Left => dx.abs() as u16,
            Direction::Down | Direction::Up => dy.abs() as u16,
        };

        Self {
            start,
            length,
            direction,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.start.translate(dx, dy);
    }

    pub fn end(&self) -> Coord {
        match self.direction {
            Direction::Right => Coord {
                x: self.start.x.saturating_add(self.length),
                y: self.start.y,
            },
            Direction::Left => Coord {
                x: self.start.x.saturating_sub(self.length),
                y: self.start.y,
            },
            Direction::Down => Coord {
                x: self.start.x,
                y: self.start.y.saturating_add(self.length),
            },
            Direction::Up => Coord {
                x: self.start.x,
                y: self.start.y.saturating_sub(self.length),
            },
        }
    }

    pub fn bounds(&self) -> Bounds {
        let end = self.end();
        Bounds {
            min: Coord {
                x: self.start.x.min(end.x),
                y: self.start.y.min(end.y),
            },
            max: Coord {
                x: self.start.x.max(end.x),
                y: self.start.y.max(end.y),
            },
        }
    }
}

pub fn calculate_bounds(segments: &[Segment]) -> Bounds {
    if segments.is_empty() {
        return Bounds {
            min: Coord { x: 0, y: 0 },
            max: Coord { x: 0, y: 0 },
        };
    }

    let mut min_x = u16::MAX;
    let mut min_y = u16::MAX;
    let mut max_x = u16::MIN;
    let mut max_y = u16::MIN;

    for segment in segments {
        let bounds = segment.bounds();
        min_x = min_x.min(bounds.min.x);
        min_y = min_y.min(bounds.min.y);
        max_x = max_x.max(bounds.max.x);
        max_y = max_y.max(bounds.max.y);
    }

    Bounds {
        min: Coord { x: min_x, y: min_y },
        max: Coord { x: max_x, y: max_y },
    }
}
