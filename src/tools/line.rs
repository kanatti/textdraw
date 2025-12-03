use crate::canvas::Canvas;
use crate::tools::DrawingTool;

pub struct LineTool {
    start: Option<(u16, u16)>,
    current: Option<(u16, u16)>,
}

impl LineTool {
    pub fn new() -> Self {
        Self {
            start: None,
            current: None,
        }
    }

    fn draw_line(&self, x1: u16, y1: u16, x2: u16, y2: u16, canvas: &mut Canvas) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();

        if dx > dy {
            // More horizontal - draw horizontal line
            self.draw_horizontal_line(x1 as i32, y1 as i32, x2 as i32, canvas);
        } else {
            // More vertical - draw vertical line
            self.draw_vertical_line(x1 as i32, y1 as i32, y2 as i32, canvas);
        }
    }

    fn draw_horizontal_line(&self, x1: i32, y: i32, x2: i32, canvas: &mut Canvas) {
        let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

        for x in start..=end {
            canvas.set(x, y, '─');
        }
    }

    fn draw_vertical_line(&self, x: i32, y1: i32, y2: i32, canvas: &mut Canvas) {
        let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

        for y in start..=end {
            canvas.set(x, y, '│');
        }
    }
}

impl DrawingTool for LineTool {
    fn on_mouse_down(&mut self, x: u16, y: u16) {
        self.start = Some((x, y));
        self.current = Some((x, y));
    }

    fn on_mouse_drag(&mut self, x: u16, y: u16) {
        self.current = Some((x, y));
    }

    fn on_mouse_up(&mut self, x: u16, y: u16, canvas: &mut Canvas) {
        if let Some((sx, sy)) = self.start {
            self.draw_line(sx, sy, x, y, canvas);
        }
        self.start = None;
        self.current = None;
    }

    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some((sx, sy)), Some((cx, cy))) = (self.start, self.current) {
            let dx = (cx as i32 - sx as i32).abs();
            let dy = (cy as i32 - sy as i32).abs();

            let mut points = vec![];

            if dx > dy {
                // Horizontal line preview
                let (start_x, end_x) = if sx <= cx { (sx, cx) } else { (cx, sx) };
                for x in start_x..=end_x {
                    points.push((x as i32, sy as i32, '─'));
                }
            } else {
                // Vertical line preview
                let (start_y, end_y) = if sy <= cy { (sy, cy) } else { (cy, sy) };
                for y in start_y..=end_y {
                    points.push((sx as i32, y as i32, '│'));
                }
            }

            points
        } else {
            vec![]
        }
    }

    fn cancel(&mut self) {
        self.start = None;
        self.current = None;
    }

    fn is_drawing(&self) -> bool {
        self.start.is_some()
    }
}

impl Default for LineTool {
    fn default() -> Self {
        Self::new()
    }
}
