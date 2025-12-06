use crate::canvas::Canvas;
use crate::types::SelectionMode;

pub struct SelectionState {
    pub mode: SelectionMode,
    pub selected_ids: Vec<usize>,         // IDs of selected elements
    pub select_start: Option<(u16, u16)>, // For drag-select box
    pub select_current: Option<(u16, u16)>,
    pub has_dragged: bool,
    pub move_start: Option<(u16, u16)>,
    pub move_offset: (i32, i32),
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            mode: SelectionMode::Idle,
            selected_ids: Vec::new(),
            select_start: None,
            select_current: None,
            has_dragged: false,
            move_start: None,
            move_offset: (0, 0),
        }
    }

    pub fn reset(&mut self) {
        self.mode = SelectionMode::Idle;
        self.selected_ids.clear();
        self.select_start = None;
        self.select_current = None;
        self.has_dragged = false;
        self.move_start = None;
        self.move_offset = (0, 0);
    }

    // Query/Read methods

    /// Get selection box points for drag-selection visualization
    pub fn get_selection_box_points(&self) -> Vec<(i32, i32, char)> {
        let mut points = vec![];

        // Only show selection box when actively selecting (dragging)
        if matches!(self.mode, SelectionMode::Selecting) {
            if let (Some((sx, sy)), Some((cx, cy))) = (self.select_start, self.select_current) {
                let (left, right) = if sx <= cx { (sx, cx) } else { (cx, sx) };
                let (top, bottom) = if sy <= cy { (sy, cy) } else { (cy, sy) };

                // Rounded corners
                points.push((left as i32, top as i32, '╭'));
                points.push((right as i32, top as i32, '╮'));
                points.push((left as i32, bottom as i32, '╰'));
                points.push((right as i32, bottom as i32, '╯'));

                // Edges
                for x in (left + 1)..right {
                    points.push((x as i32, top as i32, '─'));
                    points.push((x as i32, bottom as i32, '─'));
                }
                for y in (top + 1)..bottom {
                    points.push((left as i32, y as i32, '│'));
                    points.push((right as i32, y as i32, '│'));
                }
            }
        }

        points
    }

    /// Get IDs of selected elements
    pub fn get_selected_element_ids(&self) -> &[usize] {
        &self.selected_ids
    }

    /// Get move offset for selected elements during moving
    pub fn get_move_offset(&self) -> Option<(i32, i32)> {
        if matches!(self.mode, SelectionMode::Moving) {
            Some(self.move_offset)
        } else {
            None
        }
    }

    pub fn is_in_selection_mode(&self) -> bool {
        !matches!(self.mode, SelectionMode::Idle)
    }

    // State-only updates

    pub fn start_selection(&mut self, x: u16, y: u16) {
        self.mode = SelectionMode::Selecting;
        self.select_start = Some((x, y));
        self.select_current = Some((x, y));
        self.has_dragged = false;
    }

    pub fn update_selection(&mut self, x: u16, y: u16) {
        self.select_current = Some((x, y));
        self.has_dragged = true;
    }

    pub fn update_move_selection(&mut self, x: u16, y: u16) {
        if let Some((start_x, start_y)) = self.move_start {
            self.move_offset = (x as i32 - start_x as i32, y as i32 - start_y as i32);
        }
    }

    pub fn deselect(&mut self) {
        self.reset();
    }

    // Selection operations that need Canvas access

    pub fn finish_selection(&mut self, x: u16, y: u16, canvas: &Canvas) {
        if let Some((sx, sy)) = self.select_start {
            if !self.has_dragged || (sx == x && sy == y) {
                // Click - select single element at this position
                self.select_element_at(x as i32, y as i32, canvas);
            } else {
                // Drag - select rectangle
                self.select_rectangle(sx, sy, x, y, canvas);
            }
        }
        self.select_start = None;
        self.select_current = None;
        self.has_dragged = false;
    }

    fn select_element_at(&mut self, x: i32, y: i32, canvas: &Canvas) {
        // Find element at this position
        if let Some(element_id) = canvas.find_element_at(x, y) {
            self.selected_ids.clear();
            self.selected_ids.push(element_id);
            self.mode = SelectionMode::Selected;
        } else {
            self.mode = SelectionMode::Idle;
        }
    }

    fn select_rectangle(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, canvas: &Canvas) {
        let (left, right) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (top, bottom) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

        // Find all elements that are fully contained within selection rectangle
        let element_ids = canvas.find_elements_fully_inside_rect(
            left as i32,
            top as i32,
            right as i32,
            bottom as i32,
        );

        if !element_ids.is_empty() {
            self.selected_ids = element_ids;
            self.mode = SelectionMode::Selected;
        } else {
            self.mode = SelectionMode::Idle;
        }
    }

    /// Toggle selection of element at position (for Shift+Click additive selection)
    pub fn toggle_selection_at(&mut self, x: i32, y: i32, canvas: &Canvas) {
        // Find element at this position
        if let Some(element_id) = canvas.find_element_at(x, y) {
            // Check if already selected
            if let Some(pos) = self.selected_ids.iter().position(|&id| id == element_id) {
                // Remove from selection
                self.selected_ids.remove(pos);
            } else {
                // Add to selection
                self.selected_ids.push(element_id);
            }

            // Update mode based on whether we have selections
            if self.selected_ids.is_empty() {
                self.mode = SelectionMode::Idle;
            } else {
                self.mode = SelectionMode::Selected;
            }
        }
    }

    // Move operations

    pub fn start_move_selection(&mut self, x: u16, y: u16, canvas: &Canvas) {
        // Check if clicking on or inside any selected element's bounds
        let px = x as i32;
        let py = y as i32;

        for element_id in &self.selected_ids {
            if let Some(element) = canvas.get_element(*element_id) {
                if element.point_in_bounds(px, py) {
                    // Clicked inside a selected element's bounding box
                    self.mode = SelectionMode::Moving;
                    self.move_start = Some((x, y));
                    self.move_offset = (0, 0);
                    return;
                }
            }
        }
    }

    pub fn finish_move_selection(&mut self, canvas: &mut Canvas) {
        let dx = self.move_offset.0;
        let dy = self.move_offset.1;

        // Move all selected elements by offset
        for element_id in &self.selected_ids {
            if let Some(element) = canvas.get_element_mut(*element_id) {
                element.translate(dx, dy);
            }
        }

        self.mode = SelectionMode::Selected;
        self.move_start = None;
        self.move_offset = (0, 0);
    }

    /// Move selected elements by offset (used for arrow key movement)
    pub fn move_selected_elements(&mut self, dx: i32, dy: i32, canvas: &mut Canvas) {
        if self.selected_ids.is_empty() {
            return;
        }

        for element_id in &self.selected_ids {
            if let Some(element) = canvas.get_element_mut(*element_id) {
                element.translate(dx, dy);
            }
        }
    }

    /// Delete selected elements
    pub fn delete_selected_elements(&mut self, canvas: &mut Canvas) {
        if self.selected_ids.is_empty() {
            return;
        }

        // Remove elements from canvas
        for element_id in &self.selected_ids {
            canvas.remove_element(*element_id);
        }

        // Clear selection
        self.reset();
    }
}
