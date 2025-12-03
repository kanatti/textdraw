use crate::canvas::Canvas;
use ratatui::layout::Rect;

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
    Box,
    Arrow,
    Text,
}

impl Tool {
    pub fn all() -> Vec<Tool> {
        vec![Tool::Select, Tool::Line, Tool::Box, Tool::Arrow, Tool::Text]
    }

    pub fn name(&self) -> &str {
        match self {
            Tool::Select => "Select",
            Tool::Line => "Line",
            Tool::Box => "Box",
            Tool::Arrow => "Arrow",
            Tool::Text => "Text",
        }
    }

    pub fn key(&self) -> char {
        match self {
            Tool::Select => 's',
            Tool::Line => 'l',
            Tool::Box => 'b',
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
    // Drawing state
    pub drawing: Option<DrawingState>,
}

#[derive(Debug, Clone, Copy)]
pub struct DrawingState {
    pub start_x: u16,
    pub start_y: u16,
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
            drawing: None,
        }
    }

    pub fn start_drawing(&mut self, x: u16, y: u16) {
        self.drawing = Some(DrawingState {
            start_x: x,
            start_y: y,
        });
    }

    pub fn finish_drawing(&mut self, end_x: u16, end_y: u16) {
        if let Some(state) = self.drawing.take() {
            match self.selected_tool {
                Tool::Line => {
                    self.draw_line(state.start_x, state.start_y, end_x, end_y);
                }
                _ => {}
            }
        }
    }

    pub fn cancel_drawing(&mut self) {
        self.drawing = None;
    }

    fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();

        if dx > dy {
            // More horizontal - draw horizontal line
            self.draw_horizontal_line(x1 as i32, y1 as i32, x2 as i32);
        } else {
            // More vertical - draw vertical line
            self.draw_vertical_line(x1 as i32, y1 as i32, y2 as i32);
        }
    }

    fn draw_horizontal_line(&mut self, x1: i32, y: i32, x2: i32) {
        let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

        for x in start..=end {
            self.canvas.set(x, y, '─');
        }
    }

    fn draw_vertical_line(&mut self, x: i32, y1: i32, y2: i32) {
        let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

        for y in start..=end {
            self.canvas.set(x, y, '│');
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
    }

    pub fn select_next_tool(&mut self) {
        let tools = Tool::all();
        self.tool_index = (self.tool_index + 1) % tools.len();
        self.selected_tool = tools[self.tool_index];
    }

    pub fn select_prev_tool(&mut self) {
        let tools = Tool::all();
        self.tool_index = if self.tool_index == 0 {
            tools.len() - 1
        } else {
            self.tool_index - 1
        };
        self.selected_tool = tools[self.tool_index];
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
