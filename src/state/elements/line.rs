use crate::types::{Bounds, Coord, Direction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineElement {
    pub id: usize,
    pub name: String,
    pub segments: Vec<LineSegment>,
    pub bounds: Bounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSegment {
    pub start: Coord,
    pub length: u16,
    pub direction: Direction,
}

impl LineElement {
    pub fn new(id: usize, segments: Vec<LineSegment>) -> Self {
        let name = format!("Line {}", id + 1);
        let bounds = Self::calculate_bounds(&segments);
        Self {
            id,
            name,
            segments,
            bounds,
        }
    }

    fn calculate_bounds(segments: &[LineSegment]) -> Bounds {
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
            let sx = segment.start.x;
            let sy = segment.start.y;
            min_x = min_x.min(sx);
            min_y = min_y.min(sy);
            max_x = max_x.max(sx);
            max_y = max_y.max(sy);

            // Calculate end point
            let (ex, ey) = match segment.direction {
                Direction::Right => (sx.saturating_add(segment.length), sy),
                Direction::Left => (sx.saturating_sub(segment.length), sy),
                Direction::Down => (sx, sy.saturating_add(segment.length)),
                Direction::Up => (sx, sy.saturating_sub(segment.length)),
            };

            min_x = min_x.min(ex);
            min_y = min_y.min(ey);
            max_x = max_x.max(ex);
            max_y = max_y.max(ey);
        }

        Bounds {
            min: Coord { x: min_x, y: min_y },
            max: Coord { x: max_x, y: max_y },
        }
    }
}
