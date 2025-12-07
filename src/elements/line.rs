use super::Segment;
use super::segment;
use crate::types::{Bounds, Direction, RenderPoint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineElement {
    pub id: usize,
    pub name: String,
    pub segments: Vec<Segment>,
    pub bounds: Bounds,
}

impl LineElement {
    pub fn new(id: usize, segments: Vec<Segment>) -> Self {
        let name = format!("Line {}", id + 1);
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

    pub fn render_points(&self) -> Vec<RenderPoint> {
        let mut points = Vec::new();

        for segment in &self.segments {
            let x = segment.start.x as i32;
            let y = segment.start.y as i32;

            match segment.direction {
                Direction::Right => {
                    for i in 0..=segment.length as i32 {
                        points.push((x + i, y, '─'));
                    }
                }
                Direction::Left => {
                    for i in 0..=segment.length as i32 {
                        points.push((x - i, y, '─'));
                    }
                }
                Direction::Down => {
                    for i in 0..=segment.length as i32 {
                        points.push((x, y + i, '│'));
                    }
                }
                Direction::Up => {
                    for i in 0..=segment.length as i32 {
                        points.push((x, y - i, '│'));
                    }
                }
            }
        }

        points
    }
}
