use crate::elements::{ArrowElement, Element, Segment};
use crate::events::{ActionType, EventHandler, EventResult, MouseEvent};
use crate::geometry;
use crate::state::CanvasState;
use crate::tools::DrawingTool;
use crate::types::Coord;

pub struct ArrowTool {
    start: Option<Coord>,
    current: Option<Coord>,
}

impl ArrowTool {
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

impl EventHandler for ArrowTool {
    type State = CanvasState;
    fn handle_mouse_down(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        self.start = Some(mouse_event.get_coord());
        self.current = Some(mouse_event.get_coord());
        EventResult::Consumed
    }

    fn handle_mouse_drag(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        self.current = Some(mouse_event.get_coord());
        EventResult::Consumed
    }

    fn handle_mouse_up(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        let Some(start) = self.start else {
            return EventResult::Consumed;
        };

        let current = mouse_event.get_coord();

        // Don't create arrow if user didn't drag (single click)
        if start == current {
            self.reset();
            return EventResult::Consumed;
        }

        let id = state.get_next_id();

        // Create a single segment from start to end (arrows are like lines with arrowhead)
        let segment = Segment::from_coords(start, current);

        let arrow = ArrowElement::new(id, vec![segment]);
        state.add_element(Element::Arrow(arrow));

        self.reset();
        EventResult::Action(ActionType::FinishedDrawing)
    }
}

impl DrawingTool for ArrowTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some(start), Some(current)) = (self.start, self.current) {
            let segment = Segment::from_coords(start, current);
            let temp_arrow = ArrowElement::new(0, vec![segment]);
            geometry::arrow_points(&temp_arrow)
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

impl Default for ArrowTool {
    fn default() -> Self {
        Self::new()
    }
}
