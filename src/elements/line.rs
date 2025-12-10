use super::Segment;
use super::segment;
use crate::types::{Bounds, Direction, RenderPoint};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
        // Build a map of point -> connected directions for blending
        let mut point_connections: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();

        // Collect all points from all segments and track their connections
        for segment in &self.segments {
            let points = get_segment_points(segment);

            for i in 0..points.len() {
                let curr = points[i];
                let entry = point_connections.entry(curr).or_default();

                // Check connection to previous point
                // We need the OPPOSITE direction (where the line comes FROM)
                if i > 0 {
                    if let Some(dir) = direction_between(curr, points[i - 1]) {
                        entry.insert(dir);
                    }
                }

                // Check connection to next point
                if i < points.len() - 1 {
                    if let Some(dir) = direction_between(curr, points[i + 1]) {
                        entry.insert(dir);
                    }
                }
            }
        }

        // Convert points with their direction sets to render points with proper characters
        point_connections
            .into_iter()
            .map(|(pos, dirs)| (pos.0, pos.1, directions_to_char(&dirs)))
            .collect()
    }
}

/// Get all points for a segment
fn get_segment_points(segment: &Segment) -> Vec<(i32, i32)> {
    let x = segment.start.x as i32;
    let y = segment.start.y as i32;
    let mut points = Vec::new();

    match segment.direction {
        Direction::Right => {
            for i in 0..=segment.length as i32 {
                points.push((x + i, y));
            }
        }
        Direction::Left => {
            for i in 0..=segment.length as i32 {
                points.push((x - i, y));
            }
        }
        Direction::Down => {
            for i in 0..=segment.length as i32 {
                points.push((x, y + i));
            }
        }
        Direction::Up => {
            for i in 0..=segment.length as i32 {
                points.push((x, y - i));
            }
        }
    }

    points
}

/// Determine the direction from one point to an adjacent point
fn direction_between(from: (i32, i32), to: (i32, i32)) -> Option<Direction> {
    match (to.0 - from.0, to.1 - from.1) {
        (0, -1) => Some(Direction::Up),
        (0, 1) => Some(Direction::Down),
        (-1, 0) => Some(Direction::Left),
        (1, 0) => Some(Direction::Right),
        _ => None, // Non-adjacent points
    }
}

/// Map a set of directions to the appropriate box-drawing character
fn directions_to_char(dirs: &HashSet<Direction>) -> char {
    use Direction::*;

    let up = dirs.contains(&Up);
    let down = dirs.contains(&Down);
    let left = dirs.contains(&Left);
    let right = dirs.contains(&Right);

    match (up, down, left, right) {
        (true, true, false, false) => '│',  // Vertical line
        (false, false, true, true) => '─',  // Horizontal line
        (true, false, false, true) => '└',  // Bottom-left corner
        (true, false, true, false) => '┘',  // Bottom-right corner
        (false, true, false, true) => '┌',  // Top-left corner
        (false, true, true, false) => '┐',  // Top-right corner
        (true, true, false, true) => '├',   // T-junction left
        (true, true, true, false) => '┤',   // T-junction right
        (false, true, true, true) => '┬',   // T-junction top
        (true, false, true, true) => '┴',   // T-junction bottom
        (true, true, true, true) => '┼',    // Cross junction
        (true, false, false, false) => '│', // Single endpoint up
        (false, true, false, false) => '│', // Single endpoint down
        (false, false, true, false) => '─', // Single endpoint left
        (false, false, false, true) => '─', // Single endpoint right
        _ => '·',                           // Fallback for empty or invalid cases
    }
}
