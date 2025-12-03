use crate::canvas::Canvas;

/// Draw a line using Bresenham's algorithm
/// Automatically chooses appropriate box-drawing characters
pub fn draw_line(canvas: &mut Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    if dx > dy {
        draw_horizontal_line(canvas, x1, y1, x2);
    } else {
        draw_vertical_line(canvas, x1, y1, y2);
    }
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

/// Draw a horizontal line
pub fn draw_horizontal_line(canvas: &mut Canvas, x1: i32, y: i32, x2: i32) {
    let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

    for x in start..=end {
        canvas.set(x, y, '─');
    }
}

/// Draw a vertical line
pub fn draw_vertical_line(canvas: &mut Canvas, x: i32, y1: i32, y2: i32) {
    let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    for y in start..=end {
        canvas.set(x, y, '│');
    }
}

/// Draw a rectangle/box
pub fn draw_box(canvas: &mut Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    let (left, right) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    let (top, bottom) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

    // Corners
    canvas.set(left, top, '┌');
    canvas.set(right, top, '┐');
    canvas.set(left, bottom, '└');
    canvas.set(right, bottom, '┘');

    // Top and bottom edges
    for x in (left + 1)..right {
        canvas.set(x, top, '─');
        canvas.set(x, bottom, '─');
    }

    // Left and right edges
    for y in (top + 1)..bottom {
        canvas.set(left, y, '│');
        canvas.set(right, y, '│');
    }
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

/// Draw an arrow from (x1, y1) to (x2, y2)
pub fn draw_arrow(canvas: &mut Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Draw the line shaft and determine arrow head position
    if dx.abs() > dy.abs() {
        // Horizontal arrow - use y1 for the entire line
        draw_horizontal_line(canvas, x1, y1, x2);
        let arrow_head = if dx > 0 { '>' } else { '<' };
        canvas.set(x2, y1, arrow_head);
    } else {
        // Vertical arrow - use x1 for the entire line
        draw_vertical_line(canvas, x1, y1, y2);
        let arrow_head = if dy > 0 { 'v' } else { '^' };
        canvas.set(x1, y2, arrow_head);
    }
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
