use crate::elements::properties::{
    FieldType, HasProperties, PropertiesSpec, PropertyField, PropertySection, PropertyValue,
};
use crate::types::{Bounds, Coord, RenderPoint};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Character set for drawing rectangle borders
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxChars {
    pub top_left: char,
    pub top: char,
    pub top_right: char,
    pub side: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

/// Border type for rectangles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BorderType {
    #[default]
    Normal,
    Double,
    Thick,
    Rounded,
}

impl BorderType {
    /// Get the box drawing characters for this border type
    pub fn chars(&self) -> BoxChars {
        match self {
            BorderType::Normal => BoxChars {
                top_left: '┌',
                top: '─',
                top_right: '┐',
                side: '│',
                bottom_left: '└',
                bottom_right: '┘',
            },
            BorderType::Double => BoxChars {
                top_left: '╔',
                top: '═',
                top_right: '╗',
                side: '║',
                bottom_left: '╚',
                bottom_right: '╝',
            },
            BorderType::Thick => BoxChars {
                top_left: '┏',
                top: '━',
                top_right: '┓',
                side: '┃',
                bottom_left: '┗',
                bottom_right: '┛',
            },
            BorderType::Rounded => BoxChars {
                top_left: '╭',
                top: '─',
                top_right: '╮',
                side: '│',
                bottom_left: '╰',
                bottom_right: '╯',
            },
        }
    }

    /// Convert border type to string for property value
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderType::Normal => "Normal",
            BorderType::Double => "Double",
            BorderType::Thick => "Thick",
            BorderType::Rounded => "Rounded",
        }
    }

    /// Parse border type from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Normal" => Ok(BorderType::Normal),
            "Double" => Ok(BorderType::Double),
            "Thick" => Ok(BorderType::Thick),
            "Rounded" => Ok(BorderType::Rounded),
            _ => bail!("Invalid border type: {}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectangleElement {
    pub id: usize,
    pub name: String,
    pub start: Coord,
    pub width: u16,
    pub height: u16,
    pub bounds: Bounds,
    #[serde(default)]
    pub border_type: BorderType,
}

impl RectangleElement {
    pub fn new(id: usize, start: Coord, width: u16, height: u16) -> Self {
        let name = format!("Rectangle {}", id + 1);
        let bounds = Bounds {
            min: start,
            max: Coord {
                x: start.x.saturating_add(width),
                y: start.y.saturating_add(height),
            },
        };
        Self {
            id,
            name,
            start,
            width,
            height,
            bounds,
            border_type: BorderType::Normal,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.start.translate(dx, dy);
        self.bounds.translate(dx, dy);
    }

    pub fn render_points(&self) -> Vec<RenderPoint> {
        let mut points = vec![];
        let left = self.start.x as i32;
        let top = self.start.y as i32;
        let right = left + self.width as i32;
        let bottom = top + self.height as i32;

        // Get characters for current border type
        let chars = self.border_type.chars();

        // Corners
        points.push((left, top, chars.top_left));
        points.push((right, top, chars.top_right));
        points.push((left, bottom, chars.bottom_left));
        points.push((right, bottom, chars.bottom_right));

        // Top and bottom edges
        for x in (left + 1)..right {
            points.push((x, top, chars.top));
            points.push((x, bottom, chars.top));
        }

        // Left and right edges
        for y in (top + 1)..bottom {
            points.push((left, y, chars.side));
            points.push((right, y, chars.side));
        }

        points
    }

    /// Update bounds after modifying position or size
    fn update_bounds(&mut self) {
        self.bounds = Bounds {
            min: self.start,
            max: Coord {
                x: self.start.x.saturating_add(self.width),
                y: self.start.y.saturating_add(self.height),
            },
        };
    }
}

impl HasProperties for RectangleElement {
    fn properties_spec(&self) -> PropertiesSpec {
        let mut spec = PropertiesSpec::new();

        // Position section
        let mut position = PropertySection::new("Position");
        position.add_field(PropertyField::new(
            "x",
            "x",
            FieldType::Numeric { min: 0, max: 1000 },
        ));
        position.add_field(PropertyField::new(
            "y",
            "y",
            FieldType::Numeric { min: 0, max: 1000 },
        ));
        spec.add_section(position);

        // Size section
        let mut size = PropertySection::new("Size");
        size.add_field(PropertyField::new(
            "width",
            "width",
            FieldType::Numeric { min: 1, max: 200 },
        ));
        size.add_field(PropertyField::new(
            "height",
            "height",
            FieldType::Numeric { min: 1, max: 200 },
        ));
        spec.add_section(size);

        // Style section
        let mut style = PropertySection::new("Style");
        style.add_field(PropertyField::new(
            "border_type",
            "border-type",
            FieldType::Choice {
                options: vec![
                    "Normal".to_string(),
                    "Double".to_string(),
                    "Thick".to_string(),
                    "Rounded".to_string(),
                ],
            },
        ));
        spec.add_section(style);

        spec
    }

    fn get_property(&self, name: &str) -> Option<PropertyValue> {
        match name {
            "x" => Some(PropertyValue::Numeric(self.start.x)),
            "y" => Some(PropertyValue::Numeric(self.start.y)),
            "width" => Some(PropertyValue::Numeric(self.width)),
            "height" => Some(PropertyValue::Numeric(self.height)),
            "border_type" => Some(PropertyValue::Choice(self.border_type.as_str().to_string())),
            _ => None,
        }
    }

    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<()> {
        match name {
            "x" => {
                let new_x = value.as_numeric()?;
                self.start.x = new_x;
                self.update_bounds();
                Ok(())
            }
            "y" => {
                let new_y = value.as_numeric()?;
                self.start.y = new_y;
                self.update_bounds();
                Ok(())
            }
            "width" => {
                let new_width = value.as_numeric()?;
                if new_width == 0 {
                    bail!("Width must be greater than 0");
                }
                self.width = new_width;
                self.update_bounds();
                Ok(())
            }
            "height" => {
                let new_height = value.as_numeric()?;
                if new_height == 0 {
                    bail!("Height must be greater than 0");
                }
                self.height = new_height;
                self.update_bounds();
                Ok(())
            }
            "border_type" => {
                let type_str = value.as_choice()?;
                self.border_type = BorderType::from_str(type_str)?;
                Ok(())
            }
            _ => bail!("Unknown property: {}", name),
        }
    }
}
