use crate::canvas::Canvas;
use crate::element::{Element, TextElement};
use crate::tools::DrawingTool;
use std::collections::HashMap;

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

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl DrawingTool for TextTool {
    fn on_mouse_down(&mut self, x: u16, y: u16) {
        // Set text position and start text input mode
        self.position = Some((x, y));
        self.text.clear();
    }

    fn on_mouse_drag(&mut self, _x: u16, _y: u16) {
        // Text tool doesn't use drag
    }

    fn on_mouse_up(&mut self, _x: u16, _y: u16, canvas: &mut Canvas) {
        // Commit text to canvas as a TextElement
        if let Some((px, py)) = self.position {
            if !self.text.is_empty() {
                let mut points = HashMap::new();
                for (i, ch) in self.text.chars().enumerate() {
                    points.insert((px as i32 + i as i32, py as i32), ch);
                }
                let id = canvas.get_next_id();
                let text_elem = TextElement::new(id, (px as i32, py as i32), self.text.clone(), points);
                canvas.add_element(Element::Text(text_elem));
            }
        }
        self.position = None;
        self.text.clear();
    }

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
