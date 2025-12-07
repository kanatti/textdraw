use crate::elements::{Element, RectangleElement};
use crate::events::{ActionType, EventHandler, EventResult, MouseEvent};
use crate::geometry;
use crate::state::CanvasState;
use crate::tools::DrawingTool;
use crate::types::Coord;

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

    fn reset(&mut self) {
        self.start = None;
        self.current = None;
    }
}

impl EventHandler for RectangleTool {
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

        // Don't create rectangle if user didn't drag (single click)
        if sx == x && sy == y {
            self.reset();
            return EventResult::Consumed;
        }

        let id = state.get_next_id();

        // Calculate top-left corner and dimensions
        let left = sx.min(x);
        let top = sy.min(y);
        let width = sx.abs_diff(x);
        let height = sy.abs_diff(y);

        let rect =
            RectangleElement::new(id, Coord { x: left, y: top }, width as u16, height as u16);
        state.add_element(Element::Rectangle(rect));

        self.reset();
        EventResult::Action(ActionType::FinishedDrawing)
    }
}

impl DrawingTool for RectangleTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some((sx, sy)), Some((cx, cy))) = (self.start, self.current) {
            let left = sx.min(cx);
            let top = sy.min(cy);
            let width = sx.abs_diff(cx);
            let height = sy.abs_diff(cy);
            let temp_rect =
                RectangleElement::new(0, Coord { x: left, y: top }, width as u16, height as u16);
            geometry::box_points(&temp_rect)
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

impl Default for RectangleTool {
    fn default() -> Self {
        Self::new()
    }
}
