use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Line(LineElement),
    Rectangle(RectangleElement),
    Arrow(ArrowElement),
    Text(TextElement),
}

impl Element {
    pub fn id(&self) -> usize {
        match self {
            Element::Line(e) => e.id,
            Element::Rectangle(e) => e.id,
            Element::Arrow(e) => e.id,
            Element::Text(e) => e.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Element::Line(e) => &e.name,
            Element::Rectangle(e) => &e.name,
            Element::Arrow(e) => &e.name,
            Element::Text(e) => &e.name,
        }
    }

    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        match self {
            Element::Line(e) => e.bounds,
            Element::Rectangle(e) => e.bounds,
            Element::Arrow(e) => e.bounds,
            Element::Text(e) => e.bounds,
        }
    }

    pub fn points(&self) -> &HashMap<(i32, i32), char> {
        match self {
            Element::Line(e) => &e.points,
            Element::Rectangle(e) => &e.points,
            Element::Arrow(e) => &e.points,
            Element::Text(e) => &e.points,
        }
    }

    pub fn points_mut(&mut self) -> &mut HashMap<(i32, i32), char> {
        match self {
            Element::Line(e) => &mut e.points,
            Element::Rectangle(e) => &mut e.points,
            Element::Arrow(e) => &mut e.points,
            Element::Text(e) => &mut e.points,
        }
    }

    /// Check if a point (x, y) is inside this element
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let (x1, y1, x2, y2) = self.bounds();

        // First check bounds for quick rejection
        if x < x1 || x > x2 || y < y1 || y > y2 {
            return false;
        }

        // Then check if there's actually a character at this point
        self.points().contains_key(&(x, y))
    }

    /// Check if a point (x, y) is within the element's bounding box
    pub fn point_in_bounds(&self, x: i32, y: i32) -> bool {
        let (x1, y1, x2, y2) = self.bounds();
        x >= x1 && x <= x2 && y >= y1 && y <= y2
    }

    /// Check if this element intersects with a rectangle
    pub fn intersects_rect(&self, rx1: i32, ry1: i32, rx2: i32, ry2: i32) -> bool {
        let (ex1, ey1, ex2, ey2) = self.bounds();
        !(ex2 < rx1 || ex1 > rx2 || ey2 < ry1 || ey1 > ry2)
    }

    /// Check if this element is fully contained within a rectangle
    pub fn is_fully_inside_rect(&self, rx1: i32, ry1: i32, rx2: i32, ry2: i32) -> bool {
        let (ex1, ey1, ex2, ey2) = self.bounds();
        ex1 >= rx1 && ex2 <= rx2 && ey1 >= ry1 && ey2 <= ry2
    }

    /// Move this element by offset (dx, dy)
    pub fn translate(&mut self, dx: i32, dy: i32) {
        match self {
            Element::Line(e) => e.translate(dx, dy),
            Element::Rectangle(e) => e.translate(dx, dy),
            Element::Arrow(e) => e.translate(dx, dy),
            Element::Text(e) => e.translate(dx, dy),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineElement {
    pub id: usize,
    pub name: String,
    pub start: (i32, i32),
    pub end: (i32, i32),
    #[serde(skip)]
    pub points: HashMap<(i32, i32), char>,
    #[serde(skip)]
    pub bounds: (i32, i32, i32, i32),
}

impl LineElement {
    pub fn new(
        id: usize,
        start: (i32, i32),
        end: (i32, i32),
        points: HashMap<(i32, i32), char>,
    ) -> Self {
        let bounds = calculate_bounds(&points);
        let name = format!("Line {}", id + 1);
        Self {
            id,
            name,
            start,
            end,
            points,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.start = (self.start.0 + dx, self.start.1 + dy);
        self.end = (self.end.0 + dx, self.end.1 + dy);
        translate_points(&mut self.points, dx, dy);
        let (x1, y1, x2, y2) = self.bounds;
        self.bounds = (x1 + dx, y1 + dy, x2 + dx, y2 + dy);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectangleElement {
    pub id: usize,
    pub name: String,
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
    #[serde(skip)]
    pub points: HashMap<(i32, i32), char>,
    #[serde(skip)]
    pub bounds: (i32, i32, i32, i32),
}

impl RectangleElement {
    pub fn new(
        id: usize,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        points: HashMap<(i32, i32), char>,
    ) -> Self {
        let bounds = calculate_bounds(&points);
        let name = format!("Rectangle {}", id + 1);
        Self {
            id,
            name,
            top_left,
            bottom_right,
            points,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.top_left = (self.top_left.0 + dx, self.top_left.1 + dy);
        self.bottom_right = (self.bottom_right.0 + dx, self.bottom_right.1 + dy);
        translate_points(&mut self.points, dx, dy);
        let (x1, y1, x2, y2) = self.bounds;
        self.bounds = (x1 + dx, y1 + dy, x2 + dx, y2 + dy);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowElement {
    pub id: usize,
    pub name: String,
    pub start: (i32, i32),
    pub end: (i32, i32),
    #[serde(skip)]
    pub points: HashMap<(i32, i32), char>,
    #[serde(skip)]
    pub bounds: (i32, i32, i32, i32),
}

impl ArrowElement {
    pub fn new(
        id: usize,
        start: (i32, i32),
        end: (i32, i32),
        points: HashMap<(i32, i32), char>,
    ) -> Self {
        let bounds = calculate_bounds(&points);
        let name = format!("Arrow {}", id + 1);
        Self {
            id,
            name,
            start,
            end,
            points,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.start = (self.start.0 + dx, self.start.1 + dy);
        self.end = (self.end.0 + dx, self.end.1 + dy);
        translate_points(&mut self.points, dx, dy);
        let (x1, y1, x2, y2) = self.bounds;
        self.bounds = (x1 + dx, y1 + dy, x2 + dx, y2 + dy);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub id: usize,
    pub name: String,
    pub position: (i32, i32),
    pub text: String,
    #[serde(skip)]
    pub points: HashMap<(i32, i32), char>,
    #[serde(skip)]
    pub bounds: (i32, i32, i32, i32),
}

impl TextElement {
    pub fn new(
        id: usize,
        position: (i32, i32),
        text: String,
        points: HashMap<(i32, i32), char>,
    ) -> Self {
        let bounds = calculate_bounds(&points);
        let name = format!("Text {}", id + 1);
        Self {
            id,
            name,
            position,
            text,
            points,
            bounds,
        }
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.position = (self.position.0 + dx, self.position.1 + dy);
        translate_points(&mut self.points, dx, dy);
        let (x1, y1, x2, y2) = self.bounds;
        self.bounds = (x1 + dx, y1 + dy, x2 + dx, y2 + dy);
    }
}

// Helper functions

pub fn calculate_bounds(points: &HashMap<(i32, i32), char>) -> (i32, i32, i32, i32) {
    if points.is_empty() {
        return (0, 0, 0, 0);
    }
    let min_x = points.keys().map(|(x, _)| *x).min().unwrap();
    let max_x = points.keys().map(|(x, _)| *x).max().unwrap();
    let min_y = points.keys().map(|(_, y)| *y).min().unwrap();
    let max_y = points.keys().map(|(_, y)| *y).max().unwrap();
    (min_x, min_y, max_x, max_y)
}

fn translate_points(points: &mut HashMap<(i32, i32), char>, dx: i32, dy: i32) {
    let mut new_points = HashMap::new();
    for ((x, y), ch) in points.drain() {
        new_points.insert((x + dx, y + dy), ch);
    }
    *points = new_points;
}
