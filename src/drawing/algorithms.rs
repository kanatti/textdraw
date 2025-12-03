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

    // Top and bottom edges
    for x in left..=right {
        canvas.set(x, top, '─');
        canvas.set(x, bottom, '─');
    }

    // Left and right edges
    for y in top..=bottom {
        canvas.set(left, y, '│');
        canvas.set(right, y, '│');
    }

    // Corners
    canvas.set(left, top, '┌');
    canvas.set(right, top, '┐');
    canvas.set(left, bottom, '└');
    canvas.set(right, bottom, '┘');
}

/// Draw an arrow from (x1, y1) to (x2, y2)
pub fn draw_arrow(canvas: &mut Canvas, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Draw the line
    draw_line(canvas, x1, y1, x2, y2);

    // Add arrow head at the end
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Determine arrow head character based on direction
    let arrow_head = if dx.abs() > dy.abs() {
        // Horizontal arrow
        if dx > 0 {
            '>'
        } else {
            '<'
        }
    } else {
        // Vertical arrow
        if dy > 0 {
            'v'
        } else {
            '^'
        }
    };

    canvas.set(x2, y2, arrow_head);
}
