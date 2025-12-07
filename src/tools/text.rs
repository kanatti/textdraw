use crate::events::{ActionType, EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::state::{CanvasState, Element, TextElement};
use crate::tools::DrawingTool;
use crate::types::Coord;
use crossterm::event::KeyCode;

pub struct TextTool {
    position: Option<(u16, u16)>,
    text: String,
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            position: None,
            text: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn backspace(&mut self) {
        self.text.pop();
    }
}

impl EventHandler for TextTool {
    type State = CanvasState;

    fn handle_key_event(&mut self, state: &mut CanvasState, key_event: &KeyEvent) -> EventResult {
        // Only handle keys when we're in text input mode (position is set)
        if self.position.is_none() {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Char(c) => {
                self.add_char(c);
                EventResult::Consumed
            }
            KeyCode::Backspace => {
                self.backspace();
                EventResult::Consumed
            }
            KeyCode::Enter => {
                self.finish(state);
                EventResult::Action(ActionType::FinishedDrawing)
            }
            KeyCode::Esc => {
                // Cancel text input
                self.cancel();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(
        &mut self,
        state: &mut CanvasState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // If already in text input mode, finish current text and start new one
        if self.position.is_some() {
            self.finish(state);
            // Start new text at new position
            self.position = Some((mouse_event.column, mouse_event.row));
            self.text.clear();
            return EventResult::Action(ActionType::FinishedDrawing);
        }

        // Start text input mode at clicked position
        self.position = Some((mouse_event.column, mouse_event.row));
        self.text.clear();
        EventResult::Consumed
    }

    fn handle_mouse_drag(
        &mut self,
        _state: &mut CanvasState,
        _mouse_event: &MouseEvent,
    ) -> EventResult {
        // Text tool doesn't use drag
        EventResult::Ignored
    }

    fn handle_mouse_up(
        &mut self,
        _state: &mut CanvasState,
        _mouse_event: &MouseEvent,
    ) -> EventResult {
        // Text tool doesn't create element on mouse_up
        // Element is created when user presses Enter (via finish())
        EventResult::Consumed
    }
}

impl DrawingTool for TextTool {
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        if let Some((px, py)) = self.position {
            let mut points = vec![];
            for (i, ch) in self.text.chars().enumerate() {
                points.push((px as i32 + i as i32, py as i32, ch));
            }
            // Add cursor
            points.push((px as i32 + self.text.len() as i32, py as i32, '█'));
            points
        } else {
            vec![]
        }
    }

    fn finish(&mut self, state: &mut CanvasState) {
        if let Some((px, py)) = self.position {
            if !self.text.is_empty() {
                let id = state.get_next_id();
                let text_elem = TextElement::new(id, Coord { x: px, y: py }, self.text.clone());
                state.add_element(Element::Text(text_elem));
            }
        }
        self.position = None;
        self.text.clear();
    }

    fn cancel(&mut self) {
        self.position = None;
        self.text.clear();
    }

    fn is_drawing(&self) -> bool {
        self.position.is_some()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Default for TextTool {
    fn default() -> Self {
        Self::new()
    }
}
