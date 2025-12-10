mod arrow;
mod borders;
mod line;
mod properties;
mod rectangle;
mod segment;
mod table;
mod text;

pub use arrow::ArrowElement;
pub use borders::{BorderChars, BorderStyle};
pub use line::LineElement;
pub use properties::{
    FieldType, HasProperties, PropertiesSpec, PropertyField, PropertySection, PropertyValue,
};
pub use rectangle::RectangleElement;
pub use segment::Segment;
pub use table::TableElement;
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
            Element::Table(e) => &e.$field,
        }
    };
    ($self:expr, $method:ident($($arg:expr),*)) => {
        match $self {
            Element::Line(e) => e.$method($($arg),*),
            Element::Rectangle(e) => e.$method($($arg),*),
            Element::Arrow(e) => e.$method($($arg),*),
            Element::Text(e) => e.$method($($arg),*),
            Element::Table(e) => e.$method($($arg),*),
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Line(LineElement),
    Rectangle(RectangleElement),
    Arrow(ArrowElement),
    Text(TextElement),
    Table(TableElement),
}

impl Element {
    pub fn id(&self) -> usize {
        *delegate_element!(self, id)
    }

    pub fn name(&self) -> &str {
        delegate_element!(self, name)
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Element::Line(_) => "Line",
            Element::Rectangle(_) => "Rectangle",
            Element::Arrow(_) => "Arrow",
            Element::Text(_) => "Text",
            Element::Table(_) => "Table",
        }
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
    pub fn render_points(&self) -> Vec<(i32, i32, char)> {
        delegate_element!(self, render_points())
    }

    /// Get properties spec (returns empty spec for elements without properties)
    pub fn properties_spec(&self) -> PropertiesSpec {
        match self {
            Element::Rectangle(rect) => rect.properties_spec(),
            Element::Table(table) => table.properties_spec(),
            _ => PropertiesSpec::default(),
        }
    }

    /// Get property value by name
    pub fn get_property(&self, name: &str) -> Option<PropertyValue> {
        match self {
            Element::Rectangle(rect) => rect.get_property(name),
            Element::Table(table) => table.get_property(name),
            _ => None,
        }
    }

    /// Set property value by name
    pub fn set_property(&mut self, name: &str, value: PropertyValue) -> anyhow::Result<()> {
        match self {
            Element::Rectangle(rect) => rect.set_property(name, value),
            Element::Table(table) => table.set_property(name, value),
            _ => Ok(()), // No-op for elements without properties
        }
    }
}
