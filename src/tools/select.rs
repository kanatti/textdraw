use crate::canvas::Canvas;
use crate::tools::DrawingTool;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SelectMode {
    Idle,
    Selecting,      // Drawing selection box
    Selected,       // Has selection
    Moving,         // Moving selection
}

pub struct SelectTool {
    mode: SelectMode,
    // Selection box coordinates
    select_start: Option<(u16, u16)>,
    select_current: Option<(u16, u16)>,
    has_dragged: bool,
    // Selected content
    selection: HashMap<(i32, i32), char>,
    selection_bounds: Option<(i32, i32, i32, i32)>, // (x1, y1, x2, y2)
    // Moving state
    move_start: Option<(u16, u16)>,
    move_offset: (i32, i32),
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            mode: SelectMode::Idle,
            select_start: None,
            select_current: None,
            has_dragged: false,
            selection: HashMap::new(),
            selection_bounds: None,
            move_start: None,
            move_offset: (0, 0),
        }
    }

    fn start_selection(&mut self, x: u16, y: u16) {
        self.mode = SelectMode::Selecting;
        self.select_start = Some((x, y));
        self.select_current = Some((x, y));
        self.has_dragged = false;
    }

    fn finish_selection(&mut self, x: u16, y: u16, canvas: &mut Canvas) {
        if let Some((sx, sy)) = self.select_start {
            let (left, right) = if sx <= x { (sx, x) } else { (x, sx) };
            let (top, bottom) = if sy <= y { (sy, y) } else { (y, sy) };

            // If this was a click (no drag), select single element at point
            let is_single_click = !self.has_dragged;

            if is_single_click {
                // Single click - select element at this point and expand to connected region
                self.select_connected_region(left as i32, top as i32, canvas);
            } else {
                // Drag selection - select everything in rectangle
                self.select_rectangle(left, top, right, bottom, canvas);
            }

            self.select_start = None;
            self.select_current = None;
        }
    }

    fn select_connected_region(&mut self, x: i32, y: i32, canvas: &mut Canvas) {
        // Check if there's something at this position
        if canvas.get(x, y).is_none() {
            self.mode = SelectMode::Idle;
            return;
        }

        // For now, just select the single character
        // TODO: Could expand to select connected characters forming a shape
        self.selection.clear();
        if let Some(ch) = canvas.get(x, y) {
            self.selection.insert((0, 0), ch);
            canvas.remove(x, y);
            self.selection_bounds = Some((x, y, x, y));
            self.mode = SelectMode::Selected;
        } else {
            self.mode = SelectMode::Idle;
        }
    }

    fn select_rectangle(&mut self, left: u16, top: u16, right: u16, bottom: u16, canvas: &mut Canvas) {
        // Extract content from selection area
        self.selection.clear();
        for cy in top..=bottom {
            for cx in left..=right {
                if let Some(ch) = canvas.get(cx as i32, cy as i32) {
                    self.selection.insert((cx as i32 - left as i32, cy as i32 - top as i32), ch);
                    // Clear from canvas
                    canvas.remove(cx as i32, cy as i32);
                }
            }
        }

        if !self.selection.is_empty() {
            self.selection_bounds = Some((left as i32, top as i32, right as i32, bottom as i32));
            self.mode = SelectMode::Selected;
        } else {
            self.mode = SelectMode::Idle;
        }
    }

    fn start_move(&mut self, x: u16, y: u16) {
        if let Some((x1, y1, x2, y2)) = self.selection_bounds {
            // Check if click is within selection bounds
            let cx = x as i32;
            let cy = y as i32;
            if cx >= x1 && cx <= x2 && cy >= y1 && cy <= y2 {
                self.mode = SelectMode::Moving;
                self.move_start = Some((x, y));
                self.move_offset = (0, 0);
            }
        }
    }

    fn update_move(&mut self, x: u16, y: u16) {
        if let Some((start_x, start_y)) = self.move_start {
            self.move_offset = (x as i32 - start_x as i32, y as i32 - start_y as i32);
        }
    }

    fn finish_move(&mut self, canvas: &mut Canvas) {
        if let Some((x1, y1, x2, y2)) = self.selection_bounds {
            let new_x1 = x1 + self.move_offset.0;
            let new_y1 = y1 + self.move_offset.1;
            let new_x2 = x2 + self.move_offset.0;
            let new_y2 = y2 + self.move_offset.1;

            // Place selection at new location
            for ((rel_x, rel_y), ch) in &self.selection {
                canvas.set(new_x1 + rel_x, new_y1 + rel_y, *ch);
            }

            self.selection_bounds = Some((new_x1, new_y1, new_x2, new_y2));
            self.mode = SelectMode::Selected;
            self.move_start = None;
            self.move_offset = (0, 0);
        }
    }

    fn deselect(&mut self, canvas: &mut Canvas) {
        // Place selection back on canvas
        if let Some((x1, y1, _, _)) = self.selection_bounds {
            for ((rel_x, rel_y), ch) in &self.selection {
                canvas.set(x1 + rel_x, y1 + rel_y, *ch);
            }
        }

        self.selection.clear();
        self.selection_bounds = None;
        self.mode = SelectMode::Idle;
    }
}

impl DrawingTool for SelectTool {
    fn on_mouse_down(&mut self, x: u16, y: u16) {
        match self.mode {
            SelectMode::Idle => {
                self.start_selection(x, y);
            }
            SelectMode::Selected => {
                self.start_move(x, y);
            }
            _ => {}
        }
    }

    fn on_mouse_drag(&mut self, x: u16, y: u16) {
        match self.mode {
            SelectMode::Selecting => {
                self.select_current = Some((x, y));
                self.has_dragged = true;
            }
            SelectMode::Moving => {
                self.update_move(x, y);
            }
            _ => {}
        }
    }

    fn on_mouse_up(&mut self, x: u16, y: u16, canvas: &mut Canvas) {
        match self.mode {
            SelectMode::Selecting => {
                self.finish_selection(x, y, canvas);
            }
            SelectMode::Moving => {
                self.finish_move(canvas);
            }
            _ => {}
        }
    }

    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        let mut points = vec![];

        match self.mode {
            SelectMode::Selecting => {
                // Show selection box with rounded corners
                if let (Some((sx, sy)), Some((cx, cy))) = (self.select_start, self.select_current) {
                    let (left, right) = if sx <= cx { (sx, cx) } else { (cx, sx) };
                    let (top, bottom) = if sy <= cy { (sy, cy) } else { (cy, sy) };

                    // Draw corners
                    points.push((left as i32, top as i32, '╭'));
                    points.push((right as i32, top as i32, '╮'));
                    points.push((left as i32, bottom as i32, '╰'));
                    points.push((right as i32, bottom as i32, '╯'));

                    // Draw horizontal edges
                    for x in (left + 1)..right {
                        points.push((x as i32, top as i32, '─'));
                        points.push((x as i32, bottom as i32, '─'));
                    }

                    // Draw vertical edges
                    for y in (top + 1)..bottom {
                        points.push((left as i32, y as i32, '│'));
                        points.push((right as i32, y as i32, '│'));
                    }
                }
            }
            SelectMode::Selected | SelectMode::Moving => {
                if let Some((x1, y1, x2, y2)) = self.selection_bounds {
                    let offset_x = if self.mode == SelectMode::Moving { self.move_offset.0 } else { 0 };
                    let offset_y = if self.mode == SelectMode::Moving { self.move_offset.1 } else { 0 };

                    let new_x1 = x1 + offset_x;
                    let new_y1 = y1 + offset_y;
                    let new_x2 = x2 + offset_x;
                    let new_y2 = y2 + offset_y;

                    // Draw corners
                    points.push((new_x1, new_y1, '╭'));
                    points.push((new_x2, new_y1, '╮'));
                    points.push((new_x1, new_y2, '╰'));
                    points.push((new_x2, new_y2, '╯'));

                    // Draw horizontal edges
                    for x in (new_x1 + 1)..new_x2 {
                        points.push((x, new_y1, '─'));
                        points.push((x, new_y2, '─'));
                    }

                    // Draw vertical edges
                    for y in (new_y1 + 1)..new_y2 {
                        points.push((new_x1, y, '│'));
                        points.push((new_x2, y, '│'));
                    }

                    // Show selection content
                    for ((rel_x, rel_y), ch) in &self.selection {
                        points.push((x1 + rel_x + offset_x, y1 + rel_y + offset_y, *ch));
                    }
                }
            }
            _ => {}
        }

        points
    }

    fn cancel(&mut self) {
        self.mode = SelectMode::Idle;
        self.select_start = None;
        self.select_current = None;
        self.has_dragged = false;
        self.selection.clear();
        self.selection_bounds = None;
        self.move_start = None;
        self.move_offset = (0, 0);
    }

    fn is_drawing(&self) -> bool {
        !matches!(self.mode, SelectMode::Idle)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Default for SelectTool {
    fn default() -> Self {
        Self::new()
    }
}
