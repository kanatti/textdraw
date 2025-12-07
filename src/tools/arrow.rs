use crate::elements::{ArrowElement, Element, Segment};
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

pub struct ArrowTool {
    segments: Vec<Segment>,      // Completed segments in current drawing
    current_start: Option<Coord>, // Start of segment being drawn
    current_end: Option<Coord>,   // Current cursor position
    state: DrawingState,
}

impl ArrowTool {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            current_start: None,
            current_end: None,
            state: DrawingState::Idle,
        }
    }

    fn reset(&mut self) {
        self.segments.clear();
        self.current_start = None;
        self.current_end = None;
        self.state = DrawingState::Idle;
    }
}

impl EventHandler for ArrowTool {
    type State = CanvasState;
    fn handle_mouse_down(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        match self.state {
            DrawingState::Idle => {
                // First click - anchor the start point
                self.current_start = Some(mouse_event.get_coord());
                self.current_end = Some(mouse_event.get_coord());
                self.state = DrawingState::Anchored;
                EventResult::Consumed
            }
            DrawingState::Anchored => {
                let Some(start) = self.current_start else {
                    self.reset();
                    return EventResult::Consumed;
                };

                let end = mouse_event.get_coord();

                // Don't add segment if start and end are the same
                if start == end {
                    return EventResult::Consumed;
                }

                // Check if Shift is pressed - add segment and continue drawing
                if mouse_event.is_shift() {
                    let segment = Segment::from_coords(start, end);

                    // Start next segment from the actual end of the created segment
                    // (not the clicked point, since from_coords picks dominant axis)
                    let actual_end = segment.end();
                    self.segments.push(segment);

                    self.current_start = Some(actual_end);
                    self.current_end = Some(actual_end);

                    return EventResult::Consumed;
                }

                // Regular click - finalize the arrow
                let segment = Segment::from_coords(start, end);
                self.segments.push(segment);

                // Create arrow element with all segments
                if !self.segments.is_empty() {
                    let id = state.get_next_id();
                    let arrow = ArrowElement::new(id, self.segments.clone());
                    state.add_element(Element::Arrow(arrow));
                }

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
            self.current_end = Some(mouse_event.get_coord());
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
        self.current_end = Some(mouse_event.get_coord());
        EventResult::Consumed
    }

    fn handle_mouse_up(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // Only finalize on mouse_up if we're in Dragging mode (drag-and-drop)
        if self.state != DrawingState::Dragging {
            return EventResult::Consumed;
        }

        let Some(start) = self.current_start else {
            self.reset();
            return EventResult::Consumed;
        };

        let end = mouse_event.get_coord();

        // Don't create arrow if user didn't drag (start == end)
        if start == end {
            self.reset();
            return EventResult::Consumed;
        }

        // Add the dragged segment to any existing segments
        let segment = Segment::from_coords(start, end);
        self.segments.push(segment);

        // Create arrow with all segments (both previously added and this dragged one)
        if !self.segments.is_empty() {
            let id = state.get_next_id();
            let arrow = ArrowElement::new(id, self.segments.clone());
            state.add_element(Element::Arrow(arrow));
        }

        self.reset();
        EventResult::Action(ActionType::FinishedDrawing)
    }
}

impl DrawingTool for ArrowTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        // Build preview with all completed segments plus current segment
        let mut preview_segments = self.segments.clone();

        // Add current segment being drawn
        if let (Some(start), Some(end)) = (self.current_start, self.current_end) {
            if start != end {
                preview_segments.push(Segment::from_coords(start, end));
            }
        }

        if preview_segments.is_empty() {
            return vec![];
        }

        let temp_arrow = ArrowElement::new(0, preview_segments);
        temp_arrow.render_points()
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
        self.current_start.is_some()
    }
}

impl Default for ArrowTool {
    fn default() -> Self {
        Self::new()
    }
}
