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
}
