use crate::types::{Bounds, Coord, RenderPoint};
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

    pub fn render_points(&self) -> Vec<RenderPoint> {
        let mut points = vec![];
        let left = self.start.x as i32;
        let top = self.start.y as i32;
        let right = left + self.width as i32;
        let bottom = top + self.height as i32;

        // Corners
        points.push((left, top, '┌'));
        points.push((right, top, '┐'));
        points.push((left, bottom, '└'));
        points.push((right, bottom, '┘'));

        // Top and bottom edges
        for x in (left + 1)..right {
            points.push((x, top, '─'));
            points.push((x, bottom, '─'));
        }

        // Left and right edges
        for y in (top + 1)..bottom {
            points.push((left, y, '│'));
            points.push((right, y, '│'));
        }

        points
    }
}
