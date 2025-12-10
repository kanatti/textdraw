use crate::elements::{Element, RectangleElement};
use crate::events::{ActionType, EventHandler, EventResult, MouseEvent};
use crate::state::CanvasState;
use crate::tools::DrawingTool;
use crate::types::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawingState {
    Idle,
    Anchored, // First click done, waiting for second click (click-move-click)
    Dragging, // User is actively dragging (drag-and-drop)
}

pub struct RectangleTool {
    start: Option<(u16, u16)>,
    current: Option<(u16, u16)>,
    state: DrawingState,
}

impl RectangleTool {
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

impl EventHandler for RectangleTool {
    type State = CanvasState;
    fn handle_mouse_down(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        match self.state {
            DrawingState::Idle => {
                // First click - anchor the start point
                self.start = Some((mouse_event.column, mouse_event.row));
                self.current = Some((mouse_event.column, mouse_event.row));
                self.state = DrawingState::Anchored;
                EventResult::Consumed
            }
            DrawingState::Anchored => {
                // Second click - finalize the rectangle (click-move-click mode)
                let Some((sx, sy)) = self.start else {
                    self.reset();
                    return EventResult::Consumed;
                };

                let x = mouse_event.column;
                let y = mouse_event.row;

                // Don't create rectangle if start and end are the same
                if sx == x && sy == y {
                    self.reset();
                    return EventResult::Consumed;
                }

                let id = state.get_next_id();
                let left = sx.min(x);
                let top = sy.min(y);
                let width = sx.abs_diff(x);
                let height = sy.abs_diff(y);

                let rect = RectangleElement::new(
                    id,
                    Coord { x: left, y: top },
                    width as u16,
                    height as u16,
                );
                state.add_element(Element::Rectangle(rect));

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
            self.current = Some((mouse_event.column, mouse_event.row));
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
        self.current = Some((mouse_event.column, mouse_event.row));
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

        let Some((sx, sy)) = self.start else {
            self.reset();
            return EventResult::Consumed;
        };

        let x = mouse_event.column;
        let y = mouse_event.row;

        // Don't create rectangle if user didn't drag (start == end)
        if sx == x && sy == y {
            self.reset();
            return EventResult::Consumed;
        }

        let id = state.get_next_id();
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
            temp_rect.render_points()
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
