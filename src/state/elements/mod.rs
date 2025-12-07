mod arrow;
mod line;
mod rectangle;
mod segment;
mod text;

pub use arrow::ArrowElement;
pub use line::LineElement;
pub use rectangle::RectangleElement;
pub use segment::{Segment, calculate_bounds as calculate_segment_bounds};
pub use text::TextElement;

use crate::types::Bounds;
use serde::{Deserialize, Serialize};

macro_rules! delegate_element {
    ($self:expr, $field:ident) => {
        match $self {
            Element::Line(e) => &e.$field,
            Element::Rectangle(e) => &e.$field,
            Element::Arrow(e) => &e.$field,
            Element::Text(e) => &e.$field,
        }
    };
    ($self:expr, $method:ident($($arg:expr),*)) => {
        match $self {
            Element::Line(e) => e.$method($($arg),*),
            Element::Rectangle(e) => e.$method($($arg),*),
            Element::Arrow(e) => e.$method($($arg),*),
            Element::Text(e) => e.$method($($arg),*),
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Line(LineElement),
    Rectangle(RectangleElement),
    Arrow(ArrowElement),
    Text(TextElement),
}

impl Element {
    pub fn id(&self) -> usize {
        *delegate_element!(self, id)
    }

    pub fn name(&self) -> &str {
        delegate_element!(self, name)
    }

    pub fn bounds(&self) -> Bounds {
        *delegate_element!(self, bounds)
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        delegate_element!(self, translate(dx, dy))
    }

    /// Check if a point is within the element's bounding box
    pub fn point_in_bounds(&self, x: i32, y: i32) -> bool {
        let bounds = self.bounds();
        x >= bounds.min.x as i32
            && x <= bounds.max.x as i32
            && y >= bounds.min.y as i32
            && y <= bounds.max.y as i32
    }

    /// Check if a point contains actual content (for precise selection)
    /// For now, just use bounds check - can be refined later
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.point_in_bounds(x, y)
    }

    /// Check if element intersects with rectangle
    pub fn intersects_rect(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let bounds = self.bounds();
        let ex1 = bounds.min.x as i32;
        let ey1 = bounds.min.y as i32;
        let ex2 = bounds.max.x as i32;
        let ey2 = bounds.max.y as i32;
        !(ex2 < x1 || ex1 > x2 || ey2 < y1 || ey1 > y2)
    }

    /// Check if element is fully inside rectangle
    pub fn is_fully_inside_rect(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let bounds = self.bounds();
        let ex1 = bounds.min.x as i32;
        let ey1 = bounds.min.y as i32;
        let ex2 = bounds.max.x as i32;
        let ey2 = bounds.max.y as i32;
        ex1 >= x1 && ex2 <= x2 && ey1 >= y1 && ey2 <= y2
    }

    /// Generate renderable points for this element
    pub fn points(&self) -> std::collections::HashMap<(i32, i32), char> {
        match self {
            Element::Line(line) => {
                let mut points = std::collections::HashMap::new();
                for segment in &line.segments {
                    let start_x = segment.start.x as i32;
                    let start_y = segment.start.y as i32;
                    let end = segment.end();
                    let end_x = end.x as i32;
                    let end_y = end.y as i32;
                    let segment_points =
                        crate::geometry::generate_line_points(start_x, start_y, end_x, end_y);
                    points.extend(segment_points);
                }
                points
            }
            Element::Rectangle(rect) => {
                let x1 = rect.start.x as i32;
                let y1 = rect.start.y as i32;
                let x2 = (rect.start.x + rect.width) as i32;
                let y2 = (rect.start.y + rect.height) as i32;
                crate::geometry::generate_box_points(x1, y1, x2, y2)
            }
            Element::Arrow(arrow) => {
                let mut points = std::collections::HashMap::new();
                for segment in &arrow.segments {
                    let start_x = segment.start.x as i32;
                    let start_y = segment.start.y as i32;
                    let end = segment.end();
                    let end_x = end.x as i32;
                    let end_y = end.y as i32;
                    let segment_points =
                        crate::geometry::generate_arrow_points(start_x, start_y, end_x, end_y);
                    points.extend(segment_points);
                }
                points
            }
            Element::Text(text) => {
                let mut points = std::collections::HashMap::new();
                for (i, ch) in text.text.chars().enumerate() {
                    points.insert(
                        (text.position.x as i32 + i as i32, text.position.y as i32),
                        ch,
                    );
                }
                points
            }
        }
    }
}
