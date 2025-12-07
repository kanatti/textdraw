use crate::element::{Element, LineElement};
use crate::events::{EventHandler, EventResult};
use crate::geometry;
use crate::state::CanvasState;
use crate::tools::DrawingTool;
use crossterm::event::MouseEvent;

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
}

impl EventHandler for LineTool {
    type State = CanvasState;

    fn handle_mouse_down(&mut self, _state: &mut CanvasState, mouse_event: &MouseEvent) -> EventResult {
        self.start = Some((mouse_event.column, mouse_event.row));
        self.current = Some((mouse_event.column, mouse_event.row));
        EventResult::Consumed
    }

    fn handle_mouse_drag(&mut self, _state: &mut CanvasState, mouse_event: &MouseEvent) -> EventResult {
        self.current = Some((mouse_event.column, mouse_event.row));
        EventResult::Consumed
    }

    fn handle_mouse_up(&mut self, state: &mut CanvasState, mouse_event: &MouseEvent) -> EventResult {
        if let Some((sx, sy)) = self.start {
            let x = mouse_event.column;
            let y = mouse_event.row;
            // Only create line if the user actually dragged (not a single click)
            if sx != x || sy != y {
                let points =
                    geometry::generate_line_points(sx as i32, sy as i32, x as i32, y as i32);
                let id = state.get_next_id();
                let line =
                    LineElement::new(id, (sx as i32, sy as i32), (x as i32, y as i32), points);
                state.add_element(Element::Line(line));
            }
        }
        self.start = None;
        self.current = None;
        EventResult::Consumed
    }
}

impl DrawingTool for LineTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some((sx, sy)), Some((cx, cy))) = (self.start, self.current) {
            geometry::line_preview_points(sx as i32, sy as i32, cx as i32, cy as i32)
        } else {
            vec![]
        }
    }

    fn finish(&mut self, state: &mut CanvasState) {
        if let (Some((sx, sy)), Some((cx, cy))) = (self.start, self.current) {
            if sx != cx || sy != cy {
                let points =
                    geometry::generate_line_points(sx as i32, sy as i32, cx as i32, cy as i32);
                let id = state.get_next_id();
                let line =
                    LineElement::new(id, (sx as i32, sy as i32), (cx as i32, cy as i32), points);
                state.add_element(Element::Line(line));
            }
        }
        self.start = None;
        self.current = None;
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

impl Default for LineTool {
    fn default() -> Self {
        Self::new()
    }
}
