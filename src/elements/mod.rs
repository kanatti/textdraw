mod arrow;
mod line;
mod rectangle;
mod segment;
mod text;

pub use arrow::ArrowElement;
pub use line::LineElement;
pub use rectangle::RectangleElement;
pub use segment::Segment;
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
    pub fn points(&self) -> Vec<(i32, i32, char)> {
        match self {
            Element::Line(line) => crate::geometry::line_points(line),
            Element::Rectangle(rect) => crate::geometry::box_points(rect),
            Element::Arrow(arrow) => crate::geometry::arrow_points(arrow),
            Element::Text(text) => {
                let mut points = Vec::new();
                for (i, ch) in text.text.chars().enumerate() {
                    points.push((
                        text.position.x as i32 + i as i32,
                        text.position.y as i32,
                        ch,
                    ));
                }
                points
            }
        }
    }
}
