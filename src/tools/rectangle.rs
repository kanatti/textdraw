use crate::canvas::Canvas;
use crate::drawing::algorithms;
use crate::tools::DrawingTool;

pub struct RectangleTool {
    start: Option<(u16, u16)>,
    current: Option<(u16, u16)>,
}

impl RectangleTool {
    pub fn new() -> Self {
        Self {
            start: None,
            current: None,
        }
    }
}

impl DrawingTool for RectangleTool {
    fn on_mouse_down(&mut self, x: u16, y: u16) {
        self.start = Some((x, y));
        self.current = Some((x, y));
    }

    fn on_mouse_drag(&mut self, x: u16, y: u16) {
        self.current = Some((x, y));
    }

    fn on_mouse_up(&mut self, x: u16, y: u16, canvas: &mut Canvas) {
        if let Some((sx, sy)) = self.start {
            algorithms::draw_box(canvas, sx as i32, sy as i32, x as i32, y as i32);
        }
        self.start = None;
        self.current = None;
    }

    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some((sx, sy)), Some((cx, cy))) = (self.start, self.current) {
            algorithms::box_preview_points(sx as i32, sy as i32, cx as i32, cy as i32)
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Default for RectangleTool {
    fn default() -> Self {
        Self::new()
    }
}
