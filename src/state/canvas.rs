use crate::file::DiagramFile;
use crate::state::Element;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Represents the drawing canvas with element-based storage
pub struct CanvasState {
    elements: Vec<Element>,
    next_id: usize,
}

impl CanvasState {
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

    /// Build a render map of all elements for efficient rendering
    /// Returns HashMap of (x, y) -> char
    pub fn build_render_map(&self) -> HashMap<(i32, i32), char> {
        let mut render_map = HashMap::new();
        for element in &self.elements {
            let points = element.points();
            for (x, y, ch) in points {
                render_map.insert((x, y), ch);
            }
        }
        render_map
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

    /// Increment and return next ID
    pub fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Check if canvas is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the bounding box of all elements (min_x, min_y, max_x, max_y)
    /// Returns (0, 0, 0, 0) if canvas is empty
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        if self.elements.is_empty() {
            return (0, 0, 0, 0);
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for element in &self.elements {
            let bounds = element.bounds();
            min_x = min_x.min(bounds.min.x as i32);
            min_y = min_y.min(bounds.min.y as i32);
            max_x = max_x.max(bounds.max.x as i32);
            max_y = max_y.max(bounds.max.y as i32);
        }

        (min_x, min_y, max_x, max_y)
    }

    /// Save the canvas to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let diagram = DiagramFile::new(self.elements.clone(), self.next_id);
        diagram.save(path)
    }

    /// Load the canvas from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let diagram = DiagramFile::load(path)?;

        self.elements = diagram.elements;
        self.next_id = diagram.next_id;

        Ok(())
    }
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::new()
    }
}
