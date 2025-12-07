use crate::elements::{Element, LineElement, Segment};
use crate::events::{ActionType, EventHandler, EventResult, MouseEvent};
use crate::state::CanvasState;
use crate::tools::DrawingTool;
use crate::types::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawingState {
    Idle,
    Anchored,  // First click done, waiting for second click (click-move-click)
    Dragging,  // User is actively dragging (drag-and-drop)
}

pub struct LineTool {
    start: Option<Coord>,
    current: Option<Coord>,
    state: DrawingState,
}

impl LineTool {
    pub fn new() -> Self {
        Self {
            start: None,
            current: None,
            state: DrawingState::Idle,
        }
    }

    fn reset(&mut self) {
        self.start = None;
        self.current = None;
        self.state = DrawingState::Idle;
    }
}

impl EventHandler for LineTool {
    type State = CanvasState;

    fn handle_mouse_down(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        match self.state {
            DrawingState::Idle => {
                // First click - anchor the start point
                self.start = Some(mouse_event.get_coord());
                self.current = Some(mouse_event.get_coord());
                self.state = DrawingState::Anchored;
                EventResult::Consumed
            }
            DrawingState::Anchored => {
                // Second click - finalize the line (click-move-click mode)
                let Some(start) = self.start else {
                    self.reset();
                    return EventResult::Consumed;
                };

                let current = mouse_event.get_coord();

                // Don't create line if start and end are the same
                if start == current {
                    self.reset();
                    return EventResult::Consumed;
                }

                let id = state.get_next_id();
                let segment = Segment::from_coords(start, current);
                let line = LineElement::new(id, vec![segment]);
                state.add_element(Element::Line(line));

                self.reset();
                EventResult::Action(ActionType::FinishedDrawing)
            }
            DrawingState::Dragging => {
                // Shouldn't happen, but reset just in case
                self.reset();
                EventResult::Consumed
            }
        }
    }

    fn handle_mouse_moved(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // Update preview when in click-move-click mode
        if self.state == DrawingState::Anchored {
            self.current = Some(mouse_event.get_coord());
            EventResult::Consumed
        } else {
            EventResult::Ignored
        }
    }

    fn handle_mouse_drag(
        &mut self,
        _state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // Switch to dragging mode and update preview
        if self.state == DrawingState::Anchored {
            self.state = DrawingState::Dragging;
        }
        self.current = Some(mouse_event.get_coord());
        EventResult::Consumed
    }

    fn handle_mouse_up(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // Only finalize on mouse_up if we're in Dragging mode
        if self.state != DrawingState::Dragging {
            return EventResult::Consumed;
        }

        let Some(start) = self.start else {
            self.reset();
            return EventResult::Consumed;
        };

        let current = mouse_event.get_coord();

        // Don't create line if user didn't drag (start == end)
        if start == current {
            self.reset();
            return EventResult::Consumed;
        }

        let id = state.get_next_id();
        let segment = Segment::from_coords(start, current);
        let line = LineElement::new(id, vec![segment]);
        state.add_element(Element::Line(line));

        self.reset();
        EventResult::Action(ActionType::FinishedDrawing)
    }
}

impl DrawingTool for LineTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let (Some(start), Some(current)) = (self.start, self.current) {
            let segment = Segment::from_coords(start, current);
            let temp_line = LineElement::new(0, vec![segment]);
            temp_line.render_points()
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
