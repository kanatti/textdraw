use std::collections::HashMap;

/// Represents the drawing canvas with sparse storage
pub struct Canvas {
    cells: HashMap<(i32, i32), char>,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: HashMap::new(),
            width,
            height,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        self.cells.get(&(x, y)).copied()
    }

    pub fn set(&mut self, x: i32, y: i32, ch: char) {
        if ch == ' ' {
            self.cells.remove(&(x, y));
        } else {
            self.cells.insert((x, y), ch);
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(100, 100)
    }
}
