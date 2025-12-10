use crate::elements::borders::BorderStyle;
use crate::elements::properties::{HasProperties, PropertiesSpec, PropertyValue};
use crate::types::{Bounds, Coord, RenderPoint};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableElement {
    pub id: usize,
    pub name: String,
    pub start: Coord,
    pub width: u16,
    pub height: u16,
    pub bounds: Bounds,

    // Structure
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<String>>,

    // Styling
    #[serde(default)]
    pub header_border: BorderStyle,
    #[serde(default)]
    pub body_border: BorderStyle,
}

impl TableElement {
    /// Fixed cell dimensions
    pub const CELL_WIDTH: u16 = 12;
    pub const CELL_HEIGHT: u16 = 2;

    /// Calculate how many rows and columns fit in the given width/height
    pub fn calculate_dimensions(width: u16, height: u16) -> (usize, usize) {
        // Each cell needs CELL_WIDTH + 1 border, plus 1 border at the start
        // cols * (CELL_WIDTH + 1) + 1 = width
        // cols = (width - 1) / (CELL_WIDTH + 1)
        let cols = ((width.saturating_sub(1)) / (Self::CELL_WIDTH + 1)).max(1) as usize;
        let rows = ((height.saturating_sub(1)) / (Self::CELL_HEIGHT + 1)).max(1) as usize;
        (rows, cols)
    }

    /// Calculate the actual width/height needed for the given rows/cols
    pub fn calculate_size(rows: usize, cols: usize) -> (u16, u16) {
        let width = (cols as u16 * (Self::CELL_WIDTH + 1)) + 1;
        let height = (rows as u16 * (Self::CELL_HEIGHT + 1)) + 1;
        (width, height)
    }

    /// Create a new table from a dragged area (auto-calculates rows/cols)
    pub fn new_from_drag(id: usize, start: Coord, drag_width: u16, drag_height: u16) -> Self {
        let (rows, cols) = Self::calculate_dimensions(drag_width, drag_height);
        Self::new(id, start, rows, cols)
    }

    /// Create a new table with specified rows and columns
    pub fn new(id: usize, start: Coord, rows: usize, cols: usize) -> Self {
        // Calculate actual size based on fixed cell dimensions
        let (width, height) = Self::calculate_size(rows, cols);

        // Generate default cell content
        let mut cells = Vec::new();
        for row in 0..rows {
            let mut row_cells = Vec::new();
            for col in 0..cols {
                let content = if row == 0 {
                    format!("Header {}", col + 1)
                } else {
                    format!("Cell {}", (row - 1) * cols + col + 1)
                };
                row_cells.push(content);
            }
            cells.push(row_cells);
        }

        let bounds = Bounds {
            min: start,
            max: Coord {
                x: start.x.saturating_add(width),
                y: start.y.saturating_add(height),
            },
        };

        Self {
            id,
            name: format!("Table {}", id + 1),
            start,
            width,
            height,
            bounds,
            rows,
            cols,
            cells,
            header_border: BorderStyle::Double,
            body_border: BorderStyle::Single,
        }
    }

    pub fn translate(&mut self, dx: i16, dy: i16) {
        self.start.translate(dx, dy);
        self.bounds.translate(dx, dy);
    }

    /// Calculate the width of each column based on content
    fn calculate_column_widths(&self) -> Vec<u16> {
        let mut col_widths = vec![Self::CELL_WIDTH; self.cols];

        // Find max content length in each column
        for row in &self.cells {
            for (col_idx, cell_content) in row.iter().enumerate() {
                let content_len = cell_content.chars().count() as u16;
                // Add 2 for padding on each side
                let needed_width = content_len.max(Self::CELL_WIDTH);
                if col_idx < col_widths.len() {
                    col_widths[col_idx] = col_widths[col_idx].max(needed_width);
                }
            }
        }

        col_widths
    }

    /// Get cell dimensions (may vary per column for width)
    fn cell_dimensions(&self) -> (u16, u16) {
        (Self::CELL_WIDTH, Self::CELL_HEIGHT)
    }

    /// Update table size based on current content
    pub fn update_size_from_content(&mut self) {
        let col_widths = self.calculate_column_widths();

        // Calculate total width: sum of column widths + borders between + borders at ends
        let total_width = col_widths.iter().sum::<u16>() + (self.cols as u16 + 1);
        let total_height = (self.rows as u16 * (Self::CELL_HEIGHT + 1)) + 1;

        self.width = total_width;
        self.height = total_height;
        self.update_bounds();
    }

    /// Get width for a specific column based on content
    pub fn get_column_width(&self, col_idx: usize) -> u16 {
        let col_widths = self.calculate_column_widths();
        col_widths.get(col_idx).copied().unwrap_or(Self::CELL_WIDTH)
    }

    /// Sync cell data when rows/cols change via properties
    fn sync_table_structure(&mut self) {
        let target_rows = self.rows;
        let target_cols = self.cols;

        // Recalculate table size based on new rows/cols
        let (new_width, new_height) = Self::calculate_size(target_rows, target_cols);
        self.width = new_width;
        self.height = new_height;
        self.update_bounds();

        // Adjust rows
        while self.cells.len() < target_rows {
            let row_idx = self.cells.len();
            let mut new_row = Vec::new();
            for col in 0..target_cols {
                let content = if row_idx == 0 {
                    format!("Header {}", col + 1)
                } else {
                    format!("Cell {}", (row_idx - 1) * target_cols + col + 1)
                };
                new_row.push(content);
            }
            self.cells.push(new_row);
        }
        while self.cells.len() > target_rows {
            self.cells.pop();
        }

        // Adjust cols in each row
        for (row_idx, row) in self.cells.iter_mut().enumerate() {
            while row.len() < target_cols {
                let col = row.len();
                let content = if row_idx == 0 {
                    format!("Header {}", col + 1)
                } else {
                    format!("Cell {}", (row_idx - 1) * target_cols + col + 1)
                };
                row.push(content);
            }
            while row.len() > target_cols {
                row.pop();
            }
        }
    }

    pub fn render_points(&self) -> Vec<RenderPoint> {
        let mut points = vec![];
        let x = self.start.x as i32;
        let y = self.start.y as i32;
        let row_height = Self::CELL_HEIGHT;
        let col_widths = self.calculate_column_widths();

        // First, render all cell content
        for row in 0..self.rows {
            let mut col_x = x;
            for col in 0..self.cols {
                let col_width = col_widths[col];
                let cell_y = y + (row as i32 * (row_height as i32 + 1));

                // Cell content area (inside borders)
                let content_x = col_x + 1;
                let content_y = cell_y + 1;

                // Get cell text
                let cell_text = if row < self.cells.len() && col < self.cells[row].len() {
                    &self.cells[row][col]
                } else {
                    ""
                };

                // Render cell text character by character
                for (i, ch) in cell_text.chars().enumerate() {
                    points.push((content_x + i as i32, content_y, ch));
                }

                col_x += col_width as i32 + 1;
            }
        }

        // Then render the complete border grid
        self.render_table_borders(&mut points, x, y, &col_widths, row_height);

        points
    }

    fn render_table_borders(
        &self,
        points: &mut Vec<RenderPoint>,
        x: i32,
        y: i32,
        col_widths: &[u16],
        row_height: u16,
    ) {
        let rows = self.rows;
        let cols = self.cols;

        let header_chars = self.header_border.chars();
        let body_chars = self.body_border.chars();

        // Render each row's borders
        for row in 0..rows {
            let is_header_row = row == 0;
            let chars = if is_header_row {
                &header_chars
            } else {
                &body_chars
            };

            let row_y = y + (row as i32 * (row_height as i32 + 1));

            // Top border (only for first row)
            if row == 0 {
                let mut curr_x = x;

                // Top-left corner
                points.push((curr_x, row_y, chars.top_left));
                curr_x += 1;

                // Top borders and T-junctions
                for col in 0..cols {
                    let col_width = col_widths[col];
                    // Horizontal line
                    for _ in 0..col_width {
                        points.push((curr_x, row_y, chars.horizontal));
                        curr_x += 1;
                    }

                    // T-junction or corner
                    if col < cols - 1 {
                        points.push((curr_x, row_y, chars.top_t));
                    } else {
                        points.push((curr_x, row_y, chars.top_right));
                    }
                    curr_x += 1;
                }
            }

            // Left and right borders (vertical lines for each row of cell height)
            let mut curr_x = x;
            for dy in 1..=row_height {
                points.push((curr_x, row_y + dy as i32, chars.vertical));

                // Internal vertical separators
                for col in 0..cols {
                    curr_x += col_widths[col] as i32 + 1;
                    points.push((curr_x, row_y + dy as i32, chars.vertical));
                }
                curr_x = x;
            }

            // Bottom border (for every row)
            if row < rows - 1 {
                // Internal row separator
                curr_x = x;
                let bottom_y = row_y + row_height as i32 + 1;

                if is_header_row && self.header_border != self.body_border {
                    // Different border styles: header gets full bottom border
                    points.push((curr_x, bottom_y, chars.bottom_left));
                    curr_x += 1;

                    for col in 0..cols {
                        let col_width = col_widths[col];
                        for _ in 0..col_width {
                            points.push((curr_x, bottom_y, chars.horizontal));
                            curr_x += 1;
                        }
                        if col < cols - 1 {
                            points.push((curr_x, bottom_y, chars.bottom_t));
                        } else {
                            points.push((curr_x, bottom_y, chars.bottom_right));
                        }
                        curr_x += 1;
                    }
                } else {
                    // Same border style or body-to-body: use T-junctions
                    points.push((curr_x, bottom_y, chars.left_t));
                    curr_x += 1;

                    for col in 0..cols {
                        let col_width = col_widths[col];
                        for _ in 0..col_width {
                            points.push((curr_x, bottom_y, chars.horizontal));
                            curr_x += 1;
                        }
                        if col < cols - 1 {
                            points.push((curr_x, bottom_y, chars.cross));
                        } else {
                            points.push((curr_x, bottom_y, chars.right_t));
                        }
                        curr_x += 1;
                    }
                }
            } else {
                // Last row - draw bottom border
                curr_x = x;
                let bottom_y = row_y + row_height as i32 + 1;

                points.push((curr_x, bottom_y, chars.bottom_left));
                curr_x += 1;

                for col in 0..cols {
                    let col_width = col_widths[col];
                    for _ in 0..col_width {
                        points.push((curr_x, bottom_y, chars.horizontal));
                        curr_x += 1;
                    }
                    if col < cols - 1 {
                        points.push((curr_x, bottom_y, chars.bottom_t));
                    } else {
                        points.push((curr_x, bottom_y, chars.bottom_right));
                    }
                    curr_x += 1;
                }
            }
        }
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

impl HasProperties for TableElement {
    fn properties_spec(&self) -> PropertiesSpec {
        PropertiesSpec::new()
            .section("Structure", |s| {
                s.numeric("rows", "rows", 1, 20)
                    .numeric("cols", "cols", 1, 10)
            })
            .section("Borders", |s| {
                s.choice("header_border", "header", BorderStyle::all_options())
                    .choice("body_border", "body", BorderStyle::all_options())
            })
    }

    fn get_property(&self, name: &str) -> Option<PropertyValue> {
        use PropertyValue::*;
        let value = match name {
            "rows" => Numeric(self.rows as u16),
            "cols" => Numeric(self.cols as u16),
            "header_border" => Choice(self.header_border.as_str().to_string()),
            "body_border" => Choice(self.body_border.as_str().to_string()),
            _ => return None,
        };
        Some(value)
    }

    fn set_property(&mut self, name: &str, value: PropertyValue) -> Result<()> {
        match name {
            "rows" => {
                let new_rows = value.as_numeric()? as usize;
                if new_rows == 0 || new_rows > 20 {
                    bail!("Rows must be between 1 and 20");
                }
                self.rows = new_rows;
                self.sync_table_structure();
            }
            "cols" => {
                let new_cols = value.as_numeric()? as usize;
                if new_cols == 0 || new_cols > 10 {
                    bail!("Cols must be between 1 and 10");
                }
                self.cols = new_cols;
                self.sync_table_structure();
            }
            "header_border" => {
                self.header_border = BorderStyle::from_str(value.as_choice()?)?;
            }
            "body_border" => {
                self.body_border = BorderStyle::from_str(value.as_choice()?)?;
            }
            _ => bail!("Unknown property: {}", name),
        }
        Ok(())
    }
}
