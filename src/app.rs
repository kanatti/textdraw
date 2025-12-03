use crate::canvas::Canvas;
use crate::tools::{
    arrow::ArrowTool, line::LineTool, rectangle::RectangleTool, text::TextTool, DrawingTool,
};
use ratatui::layout::Rect;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    Canvas = 0,
    Tools = 1,
    Elements = 2,
    Properties = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Select,
    Line,
    Rectangle,
    Arrow,
    Text,
}

impl Tool {
    pub fn all() -> Vec<Tool> {
        vec![Tool::Select, Tool::Line, Tool::Rectangle, Tool::Arrow, Tool::Text]
    }

    pub fn name(&self) -> &str {
        match self {
            Tool::Select => "Select",
            Tool::Line => "Line",
            Tool::Rectangle => "Rectangle",
            Tool::Arrow => "Arrow",
            Tool::Text => "Text",
        }
    }

    pub fn key(&self) -> char {
        match self {
            Tool::Select => 's',
            Tool::Line => 'l',
            Tool::Rectangle => 'r',
            Tool::Arrow => 'a',
            Tool::Text => 't',
        }
    }
}

pub struct App {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub canvas_area: Option<Rect>,
    pub active_panel: Panel,
    pub selected_tool: Tool,
    pub tool_index: usize, // For arrow key navigation
    // Store panel areas for click detection
    pub tools_area: Option<Rect>,
    pub elements_area: Option<Rect>,
    pub properties_area: Option<Rect>,
    // Drawing canvas
    pub canvas: Canvas,
    // Active tool instance (None when in Select mode)
    active_tool: Option<Box<dyn DrawingTool>>,
    // Selection state (for Select tool)
    pub selection_state: SelectionState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    Idle,
    Selecting,
    Selected,
    Moving,
}

pub struct SelectionState {
    pub mode: SelectionMode,
    pub select_start: Option<(u16, u16)>,
    pub select_current: Option<(u16, u16)>,
    pub has_dragged: bool,
    pub selection: HashMap<(i32, i32), char>,
    pub selection_bounds: Option<(i32, i32, i32, i32)>,
    pub move_start: Option<(u16, u16)>,
    pub move_offset: (i32, i32),
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            mode: SelectionMode::Idle,
            select_start: None,
            select_current: None,
            has_dragged: false,
            selection: HashMap::new(),
            selection_bounds: None,
            move_start: None,
            move_offset: (0, 0),
        }
    }

    pub fn reset(&mut self) {
        self.mode = SelectionMode::Idle;
        self.select_start = None;
        self.select_current = None;
        self.has_dragged = false;
        self.selection.clear();
        self.selection_bounds = None;
        self.move_start = None;
        self.move_offset = (0, 0);
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            canvas_area: None,
            active_panel: Panel::Canvas,
            selected_tool: Tool::Select,
            tool_index: 0,
            tools_area: None,
            elements_area: None,
            properties_area: None,
            canvas: Canvas::default(),
            active_tool: None, // No active tool when in Select mode
            selection_state: SelectionState::new(),
        }
    }

    pub fn start_drawing(&mut self, x: u16, y: u16) {
        if let Some(tool) = &mut self.active_tool {
            tool.on_mouse_down(x, y);
        }
    }

    pub fn update_drawing(&mut self, x: u16, y: u16) {
        if let Some(tool) = &mut self.active_tool {
            tool.on_mouse_drag(x, y);
        }
    }

    pub fn finish_drawing(&mut self, x: u16, y: u16) {
        if let Some(tool) = &mut self.active_tool {
            tool.on_mouse_up(x, y, &mut self.canvas);
        }
    }

    pub fn cancel_drawing(&mut self) {
        if let Some(tool) = &mut self.active_tool {
            tool.cancel();
        }
    }

    pub fn is_drawing(&self) -> bool {
        if let Some(tool) = &self.active_tool {
            tool.is_drawing()
        } else {
            false
        }
    }

    pub fn get_preview_points(&self) -> Vec<(i32, i32, char)> {
        if let Some(tool) = &self.active_tool {
            tool.preview_points()
        } else {
            vec![]
        }
    }

    pub fn is_text_input_mode(&self) -> bool {
        self.selected_tool == Tool::Text && self.is_drawing()
    }

    pub fn is_select_tool(&self) -> bool {
        self.selected_tool == Tool::Select
    }

    pub fn get_selection_box_points(&self) -> Vec<(i32, i32, char)> {
        use SelectionMode::*;
        let mut points = vec![];

        match self.selection_state.mode {
            Selecting => {
                if let (Some((sx, sy)), Some((cx, cy))) = (self.selection_state.select_start, self.selection_state.select_current) {
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
            Selected | Moving => {
                if let Some((x1, y1, x2, y2)) = self.selection_state.selection_bounds {
                    let offset_x = if self.selection_state.mode == Moving { self.selection_state.move_offset.0 } else { 0 };
                    let offset_y = if self.selection_state.mode == Moving { self.selection_state.move_offset.1 } else { 0 };

                    let new_x1 = x1 + offset_x;
                    let new_y1 = y1 + offset_y;
                    let new_x2 = x2 + offset_x;
                    let new_y2 = y2 + offset_y;

                    // Rounded corners
                    points.push((new_x1, new_y1, '╭'));
                    points.push((new_x2, new_y1, '╮'));
                    points.push((new_x1, new_y2, '╰'));
                    points.push((new_x2, new_y2, '╯'));

                    // Edges
                    for x in (new_x1 + 1)..new_x2 {
                        points.push((x, new_y1, '─'));
                        points.push((x, new_y2, '─'));
                    }
                    for y in (new_y1 + 1)..new_y2 {
                        points.push((new_x1, y, '│'));
                        points.push((new_x2, y, '│'));
                    }
                }
            }
            _ => {}
        }

        points
    }

    pub fn get_selection_content_points(&self) -> Vec<(i32, i32, char)> {
        use SelectionMode::*;
        let mut points = vec![];

        if matches!(self.selection_state.mode, Selected | Moving) {
            if let Some((x1, y1, _, _)) = self.selection_state.selection_bounds {
                let offset_x = if self.selection_state.mode == Moving { self.selection_state.move_offset.0 } else { 0 };
                let offset_y = if self.selection_state.mode == Moving { self.selection_state.move_offset.1 } else { 0 };

                for ((rel_x, rel_y), ch) in &self.selection_state.selection {
                    points.push((x1 + rel_x + offset_x, y1 + rel_y + offset_y, *ch));
                }
            }
        }

        points
    }

    // Selection mode methods
    pub fn start_selection(&mut self, x: u16, y: u16) {
        self.selection_state.mode = SelectionMode::Selecting;
        self.selection_state.select_start = Some((x, y));
        self.selection_state.select_current = Some((x, y));
        self.selection_state.has_dragged = false;
    }

    pub fn update_selection(&mut self, x: u16, y: u16) {
        self.selection_state.select_current = Some((x, y));
        self.selection_state.has_dragged = true;
    }

    pub fn finish_selection(&mut self, x: u16, y: u16) {
        if let Some((sx, sy)) = self.selection_state.select_start {
            if !self.selection_state.has_dragged || (sx == x && sy == y) {
                // Click - select single element at this position
                self.select_element_at(x as i32, y as i32);
            } else {
                // Drag - select rectangle
                self.select_rectangle(sx, sy, x, y);
            }
        }
        self.selection_state.select_start = None;
        self.selection_state.select_current = None;
        self.selection_state.has_dragged = false;
    }

    fn select_element_at(&mut self, x: i32, y: i32) {
        if self.canvas.get(x, y).is_none() {
            self.selection_state.mode = SelectionMode::Idle;
            return;
        }

        // Flood fill to find all connected characters
        self.selection_state.selection.clear();
        let mut to_visit = vec![(x, y)];
        let mut visited = std::collections::HashSet::new();

        while let Some((cx, cy)) = to_visit.pop() {
            if visited.contains(&(cx, cy)) {
                continue;
            }
            visited.insert((cx, cy));

            if let Some(ch) = self.canvas.get(cx, cy) {
                self.selection_state.selection.insert((cx, cy), ch);

                // Check all 8 neighbors (including diagonals)
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = cx + dx;
                        let ny = cy + dy;
                        if !visited.contains(&(nx, ny)) && self.canvas.get(nx, ny).is_some() {
                            to_visit.push((nx, ny));
                        }
                    }
                }
            }
        }

        if !self.selection_state.selection.is_empty() {
            // Calculate bounds
            let min_x = self.selection_state.selection.keys().map(|(x, _)| *x).min().unwrap();
            let max_x = self.selection_state.selection.keys().map(|(x, _)| *x).max().unwrap();
            let min_y = self.selection_state.selection.keys().map(|(_, y)| *y).min().unwrap();
            let max_y = self.selection_state.selection.keys().map(|(_, y)| *y).max().unwrap();

            // Convert to relative coordinates
            let mut relative_selection = HashMap::new();
            for ((abs_x, abs_y), ch) in self.selection_state.selection.drain() {
                relative_selection.insert((abs_x - min_x, abs_y - min_y), ch);
                self.canvas.remove(abs_x, abs_y);
            }
            self.selection_state.selection = relative_selection;
            self.selection_state.selection_bounds = Some((min_x, min_y, max_x, max_y));
            self.selection_state.mode = SelectionMode::Selected;
        } else {
            self.selection_state.mode = SelectionMode::Idle;
        }
    }

    fn select_rectangle(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        let (left, right) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (top, bottom) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

        self.selection_state.selection.clear();
        for cy in top..=bottom {
            for cx in left..=right {
                if let Some(ch) = self.canvas.get(cx as i32, cy as i32) {
                    self.selection_state.selection.insert((cx as i32 - left as i32, cy as i32 - top as i32), ch);
                    self.canvas.remove(cx as i32, cy as i32);
                }
            }
        }

        if !self.selection_state.selection.is_empty() {
            self.selection_state.selection_bounds = Some((left as i32, top as i32, right as i32, bottom as i32));
            self.selection_state.mode = SelectionMode::Selected;
        } else {
            self.selection_state.mode = SelectionMode::Idle;
        }
    }

    pub fn start_move_selection(&mut self, x: u16, y: u16) {
        if let Some((x1, y1, x2, y2)) = self.selection_state.selection_bounds {
            let cx = x as i32;
            let cy = y as i32;
            if cx >= x1 && cx <= x2 && cy >= y1 && cy <= y2 {
                // Remove content from canvas before moving
                for ((rel_x, rel_y), _) in &self.selection_state.selection {
                    self.canvas.remove(x1 + rel_x, y1 + rel_y);
                }

                self.selection_state.mode = SelectionMode::Moving;
                self.selection_state.move_start = Some((x, y));
                self.selection_state.move_offset = (0, 0);
            }
        }
    }

    pub fn update_move_selection(&mut self, x: u16, y: u16) {
        if let Some((start_x, start_y)) = self.selection_state.move_start {
            self.selection_state.move_offset = (x as i32 - start_x as i32, y as i32 - start_y as i32);
        }
    }

    pub fn finish_move_selection(&mut self) {
        if let Some((x1, y1, x2, y2)) = self.selection_state.selection_bounds {
            let new_x1 = x1 + self.selection_state.move_offset.0;
            let new_y1 = y1 + self.selection_state.move_offset.1;
            let new_x2 = x2 + self.selection_state.move_offset.0;
            let new_y2 = y2 + self.selection_state.move_offset.1;

            // Place selection at new location
            for ((rel_x, rel_y), ch) in &self.selection_state.selection {
                self.canvas.set(new_x1 + rel_x, new_y1 + rel_y, *ch);
            }

            self.selection_state.selection_bounds = Some((new_x1, new_y1, new_x2, new_y2));
            self.selection_state.mode = SelectionMode::Selected;
            self.selection_state.move_start = None;
            self.selection_state.move_offset = (0, 0);
        }
    }

    pub fn is_in_selection_mode(&self) -> bool {
        !matches!(self.selection_state.mode, SelectionMode::Idle)
    }

    pub fn deselect(&mut self) {
        // Place selection back on canvas
        if let Some((x1, y1, _, _)) = self.selection_state.selection_bounds {
            for ((rel_x, rel_y), ch) in &self.selection_state.selection {
                self.canvas.set(x1 + rel_x, y1 + rel_y, *ch);
            }
        }
        self.selection_state.reset();
    }

    pub fn add_text_char(&mut self, c: char) {
        if let Some(tool) = &mut self.active_tool {
            if let Some(text_tool) = tool.as_any_mut().downcast_mut::<TextTool>() {
                text_tool.add_char(c);
            }
        }
    }

    pub fn text_backspace(&mut self) {
        if let Some(tool) = &mut self.active_tool {
            if let Some(text_tool) = tool.as_any_mut().downcast_mut::<TextTool>() {
                text_tool.backspace();
            }
        }
    }

    pub fn finish_text_input(&mut self) {
        if let Some(tool) = &mut self.active_tool {
            tool.on_mouse_up(0, 0, &mut self.canvas);
        }
    }

    pub fn switch_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
    }

    pub fn update_cursor(&mut self, x: u16, y: u16) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    /// Check if a coordinate is inside a rect
    pub fn is_inside(&self, x: u16, y: u16, rect: Rect) -> bool {
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    }

    /// Detect which panel was clicked based on mouse coordinates
    pub fn detect_panel_click(&self, x: u16, y: u16) -> Option<Panel> {
        if let Some(area) = self.canvas_area {
            if self.is_inside(x, y, area) {
                return Some(Panel::Canvas);
            }
        }
        if let Some(area) = self.tools_area {
            if self.is_inside(x, y, area) {
                return Some(Panel::Tools);
            }
        }
        if let Some(area) = self.elements_area {
            if self.is_inside(x, y, area) {
                return Some(Panel::Elements);
            }
        }
        if let Some(area) = self.properties_area {
            if self.is_inside(x, y, area) {
                return Some(Panel::Properties);
            }
        }
        None
    }

    /// Detect which tool was clicked based on mouse coordinates
    pub fn detect_tool_click(&self, x: u16, y: u16) -> Option<Tool> {
        if let Some(area) = self.tools_area {
            if !self.is_inside(x, y, area) {
                return None;
            }

            // Calculate relative Y position within tools panel
            let relative_y = y.saturating_sub(area.y + 1); // +1 for border

            // Tools start at line 1 (after empty line), one tool per line
            let tool_index = relative_y.saturating_sub(1);
            let tools = Tool::all();

            if (tool_index as usize) < tools.len() {
                return Some(tools[tool_index as usize]);
            }
        }
        None
    }

    pub fn select_tool(&mut self, tool: Tool) {
        self.selected_tool = tool;
        self.tool_index = Tool::all().iter().position(|&t| t == tool).unwrap_or(0);

        // Create new tool instance based on selection (None for Select mode)
        self.active_tool = match tool {
            Tool::Select => None, // Selection is handled by selection_state, not as a tool
            Tool::Line => Some(Box::new(LineTool::new())),
            Tool::Rectangle => Some(Box::new(RectangleTool::new())),
            Tool::Arrow => Some(Box::new(ArrowTool::new())),
            Tool::Text => Some(Box::new(TextTool::new())),
        };

        // Deselect when switching away from Select tool
        if tool != Tool::Select && self.is_in_selection_mode() {
            self.deselect();
        }
    }

    pub fn select_next_tool(&mut self) {
        let tools = Tool::all();
        self.tool_index = (self.tool_index + 1) % tools.len();
        let tool = tools[self.tool_index];
        self.select_tool(tool);
    }

    pub fn select_prev_tool(&mut self) {
        let tools = Tool::all();
        self.tool_index = if self.tool_index == 0 {
            tools.len() - 1
        } else {
            self.tool_index - 1
        };
        let tool = tools[self.tool_index];
        self.select_tool(tool);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
