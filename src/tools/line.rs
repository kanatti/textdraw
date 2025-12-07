use crate::events::{ActionType, EventHandler, EventResult, MouseEvent};
use crate::geometry;
use crate::state::{CanvasState, Element, LineElement, Segment};
use crate::tools::DrawingTool;
use crate::types::Coord;

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

    fn reset(&mut self) {
        self.start = None;
        self.current = None;
    }
}

impl EventHandler for LineTool {
    type State = CanvasState;

    fn handle_mouse_down(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        self.start = Some((mouse_event.column, mouse_event.row));
        self.current = Some((mouse_event.column, mouse_event.row));
        EventResult::Consumed
    }

    fn handle_mouse_drag(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        self.current = Some((mouse_event.column, mouse_event.row));
        EventResult::Consumed
    }

    fn handle_mouse_up(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let Some((sx, sy)) = self.start else {
            return EventResult::Consumed;
        };

        let x = mouse_event.column;
        let y = mouse_event.row;

        // Don't create line if user didn't drag (single click)
        if sx == x && sy == y {
            self.reset();
            return EventResult::Consumed;
        }

        let id = state.get_next_id();

        // Create a single segment from start to end
        let segment = Segment::from_coords(Coord { x: sx, y: sy }, Coord { x, y });

        let line = LineElement::new(id, vec![segment]);
        state.add_element(Element::Line(line));

        self.reset();
        EventResult::Action(ActionType::FinishedDrawing)
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

    fn finish(&mut self, _state: &mut CanvasState) {
        // Just clear state without creating element
        // Element creation only happens on mouse_up
        self.reset();
    }

    fn cancel(&mut self) {
        self.reset();
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
