use std::collections::HashMap;

/// Generate points for a line using Bresenham's algorithm
/// Returns HashMap of (x, y) -> char
pub fn generate_line_points(x1: i32, y1: i32, x2: i32, y2: i32) -> HashMap<(i32, i32), char> {
    let points = line_preview_points(x1, y1, x2, y2);
    points.into_iter().map(|(x, y, ch)| ((x, y), ch)).collect()
}

/// Generate preview points for a line
pub fn line_preview_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32, char)> {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let mut points = vec![];

    if dx > dy {
        // Horizontal line
        let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            points.push((x, y1, '─'));
        }
    } else {
        // Vertical line
        let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            points.push((x1, y, '│'));
        }
    }

    points
}

/// Generate points for a rectangle/box
/// Returns HashMap of (x, y) -> char
pub fn generate_box_points(x1: i32, y1: i32, x2: i32, y2: i32) -> HashMap<(i32, i32), char> {
    let points = box_preview_points(x1, y1, x2, y2);
    points.into_iter().map(|(x, y, ch)| ((x, y), ch)).collect()
}

/// Generate preview points for a box
pub fn box_preview_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32, char)> {
    let mut points = vec![];
    let (left, right) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

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

/// Generate points for an arrow
/// Returns HashMap of (x, y) -> char
pub fn generate_arrow_points(x1: i32, y1: i32, x2: i32, y2: i32) -> HashMap<(i32, i32), char> {
    let points = arrow_preview_points(x1, y1, x2, y2);
    points.into_iter().map(|(x, y, ch)| ((x, y), ch)).collect()
}

/// Generate preview points for an arrow
pub fn arrow_preview_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32, char)> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut points = vec![];

    if dx.abs() > dy.abs() {
        // Horizontal arrow - all points use y1
        let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            let ch = if x == x2 {
                // Arrowhead at x2
                if dx > 0 { '>' } else { '<' }
            } else {
                '─'
            };
            points.push((x, y1, ch));
        }
    } else {
        // Vertical arrow - all points use x1
        let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            let ch = if y == y2 {
                // Arrowhead at y2
                if dy > 0 { 'v' } else { '^' }
            } else {
                '│'
            };
            points.push((x1, y, ch));
        }
    }

    points
}
