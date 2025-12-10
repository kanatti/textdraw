use crate::elements::properties::{HasProperties, PropertiesSpec, PropertyValue};
use crate::types::{Bounds, Coord, RenderPoint};
use anyhow::{Result, bail};
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

impl BoxChars {
    const fn new(
        top_left: char,
        top: char,
        top_right: char,
        side: char,
        bottom_left: char,
        bottom_right: char,
    ) -> Self {
        Self {
            top_left,
            top,
            top_right,
            side,
            bottom_left,
            bottom_right,
        }
    }
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
            BorderType::Normal => BoxChars::new('┌', '─', '┐', '│', '└', '┘'),
            BorderType::Double => BoxChars::new('╔', '═', '╗', '║', '╚', '╝'),
            BorderType::Thick => BoxChars::new('┏', '━', '┓', '┃', '┗', '┛'),
            BorderType::Rounded => BoxChars::new('╭', '─', '╮', '│', '╰', '╯'),
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
        PropertiesSpec::new()
            .section("Position", |s| {
                s.numeric("x", "x", 0, 1000).numeric("y", "y", 0, 1000)
            })
            .section("Size", |s| {
                s.numeric("width", "width", 1, 200)
                    .numeric("height", "height", 1, 200)
            })
            .section("Style", |s| {
                s.choice(
                    "border_type",
                    "border-type",
                    vec![
                        "Normal".to_string(),
                        "Double".to_string(),
                        "Thick".to_string(),
                        "Rounded".to_string(),
                    ],
                )
            })
    }

    fn get_property(&self, name: &str) -> Option<PropertyValue> {
        use PropertyValue::*;
        let value = match name {
            "x" => Numeric(self.start.x),
            "y" => Numeric(self.start.y),
            "width" => Numeric(self.width),
            "height" => Numeric(self.height),
            "border_type" => Choice(self.border_type.as_str().to_string()),
            _ => return None,
        };
        Some(value)
    }

    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<()> {
        match name {
            "x" => {
                self.start.x = value.as_numeric()?;
                self.update_bounds();
            }
            "y" => {
                self.start.y = value.as_numeric()?;
                self.update_bounds();
            }
            "width" => {
                let new_width = value.as_numeric()?;
                if new_width == 0 {
                    bail!("Width must be greater than 0");
                }
                self.width = new_width;
                self.update_bounds();
            }
            "height" => {
                let new_height = value.as_numeric()?;
                if new_height == 0 {
                    bail!("Height must be greater than 0");
                }
                self.height = new_height;
                self.update_bounds();
            }
            "border_type" => {
                self.border_type = BorderType::from_str(value.as_choice()?)?;
            }
            _ => bail!("Unknown property: {}", name),
        }
        Ok(())
    }
}
