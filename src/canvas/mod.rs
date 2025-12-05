use crate::drawing::algorithms;
use crate::element::Element;
use crate::types::DiagramFile;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Represents the drawing canvas with element-based storage
pub struct Canvas {
    elements: Vec<Element>,
    next_id: usize,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            next_id: 0,
        }
    }

    /// Add an element to the canvas and return its ID
    pub fn add_element(&mut self, element: Element) -> usize {
        let id = element.id();
        self.elements.push(element);
        id
    }

    /// Get character at position (x, y) - returns topmost element's character
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        // Iterate in reverse (top to bottom - last drawn is on top)
        for element in self.elements.iter().rev() {
            if let Some(&ch) = element.points().get(&(x, y)) {
                return Some(ch);
            }
        }
        None
    }

    /// Find the topmost element at position (x, y)
    pub fn find_element_at(&self, x: i32, y: i32) -> Option<usize> {
        for element in self.elements.iter().rev() {
            if element.contains_point(x, y) {
                return Some(element.id());
            }
        }
        None
    }

    /// Find all elements that intersect with the given rectangle
    pub fn find_elements_in_rect(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<usize> {
        self.elements
            .iter()
            .filter(|e| e.intersects_rect(x1, y1, x2, y2))
            .map(|e| e.id())
            .collect()
    }

    /// Find all elements that are fully contained within the given rectangle
    pub fn find_elements_fully_inside_rect(
        &self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Vec<usize> {
        self.elements
            .iter()
            .filter(|e| e.is_fully_inside_rect(x1, y1, x2, y2))
            .map(|e| e.id())
            .collect()
    }

    /// Get reference to an element by ID
    pub fn get_element(&self, id: usize) -> Option<&Element> {
        self.elements.iter().find(|e| e.id() == id)
    }

    /// Get mutable reference to an element by ID
    pub fn get_element_mut(&mut self, id: usize) -> Option<&mut Element> {
        self.elements.iter_mut().find(|e| e.id() == id)
    }

    /// Remove element by ID
    pub fn remove_element(&mut self, id: usize) -> Option<Element> {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == id) {
            Some(self.elements.remove(pos))
        } else {
            None
        }
    }

    /// Get all elements
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    /// Get next available ID
    pub fn next_id(&self) -> usize {
        self.next_id
    }

    /// Increment and return next ID
    pub fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Clear all elements
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Save the canvas to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let diagram = DiagramFile {
            version: env!("CARGO_PKG_VERSION").to_string(),
            elements: self.elements.clone(),
            next_id: self.next_id,
        };

        let json = serde_json::to_string_pretty(&diagram)
            .context("Failed to serialize diagram")?;

        fs::write(path.as_ref(), json)
            .context(format!("Failed to write to file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Load the canvas from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let json = fs::read_to_string(path.as_ref())
            .context(format!("Failed to read file: {}", path.as_ref().display()))?;

        let mut diagram: DiagramFile = serde_json::from_str(&json)
            .context("Failed to parse diagram file")?;

        // Rebuild points and bounds for all elements after deserialization
        for element in &mut diagram.elements {
            Self::rebuild_element(element);
        }

        self.elements = diagram.elements;
        self.next_id = diagram.next_id;

        Ok(())
    }

    /// Rebuild points and bounds for an element (used after deserialization)
    fn rebuild_element(element: &mut Element) {
        match element {
            Element::Line(line) => {
                line.points = algorithms::generate_line_points(
                    line.start.0,
                    line.start.1,
                    line.end.0,
                    line.end.1,
                );
                line.bounds = crate::element::calculate_bounds(&line.points);
            }
            Element::Rectangle(rect) => {
                rect.points = algorithms::generate_box_points(
                    rect.top_left.0,
                    rect.top_left.1,
                    rect.bottom_right.0,
                    rect.bottom_right.1,
                );
                rect.bounds = crate::element::calculate_bounds(&rect.points);
            }
            Element::Arrow(arrow) => {
                arrow.points = algorithms::generate_arrow_points(
                    arrow.start.0,
                    arrow.start.1,
                    arrow.end.0,
                    arrow.end.1,
                );
                arrow.bounds = crate::element::calculate_bounds(&arrow.points);
            }
            Element::Text(text) => {
                // Rebuild text points: place each character horizontally
                text.points = text.text
                    .chars()
                    .enumerate()
                    .map(|(i, ch)| ((text.position.0 + i as i32, text.position.1), ch))
                    .collect();
                text.bounds = crate::element::calculate_bounds(&text.points);
            }
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
