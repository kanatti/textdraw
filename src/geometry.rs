use crate::elements::{ArrowElement, LineElement, RectangleElement};
use crate::types::{Direction, RenderPoint};

/// Generate points for a line element
pub fn line_points(line: &LineElement) -> Vec<RenderPoint> {
    let mut points = Vec::new();

    for segment in &line.segments {
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

/// Generate points for a rectangle element
pub fn box_points(rect: &RectangleElement) -> Vec<RenderPoint> {
    let mut points = vec![];
    let left = rect.start.x as i32;
    let top = rect.start.y as i32;
    let right = left + rect.width as i32;
    let bottom = top + rect.height as i32;

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

/// Generate points for an arrow element
pub fn arrow_points(arrow: &ArrowElement) -> Vec<RenderPoint> {
    let mut points = Vec::new();

    for segment in &arrow.segments {
        let x = segment.start.x as i32;
        let y = segment.start.y as i32;

        match segment.direction {
            Direction::Right => {
                for i in 0..=segment.length as i32 {
                    let ch = if i == segment.length as i32 {
                        '>'
                    } else {
                        '─'
                    };
                    points.push((x + i, y, ch));
                }
            }
            Direction::Left => {
                for i in 0..=segment.length as i32 {
                    let ch = if i == segment.length as i32 {
                        '<'
                    } else {
                        '─'
                    };
                    points.push((x - i, y, ch));
                }
            }
            Direction::Down => {
                for i in 0..=segment.length as i32 {
                    let ch = if i == segment.length as i32 {
                        'v'
                    } else {
                        '│'
                    };
                    points.push((x, y + i, ch));
                }
            }
            Direction::Up => {
                for i in 0..=segment.length as i32 {
                    let ch = if i == segment.length as i32 {
                        '^'
                    } else {
                        '│'
                    };
                    points.push((x, y - i, ch));
                }
            }
        }
    }

    points
}
