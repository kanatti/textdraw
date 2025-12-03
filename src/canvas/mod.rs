use crate::element::Element;

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
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
