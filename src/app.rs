use crate::canvas::Canvas;
use crate::tools::{
    arrow::ArrowTool, line::LineTool, rectangle::RectangleTool, text::TextTool, DrawingTool,
};
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
    // Active tool instance
    active_tool: Box<dyn DrawingTool>,
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
            active_tool: Box::new(LineTool::new()),
        }
    }

    pub fn start_drawing(&mut self, x: u16, y: u16) {
        self.active_tool.on_mouse_down(x, y);
    }

    pub fn update_drawing(&mut self, x: u16, y: u16) {
        self.active_tool.on_mouse_drag(x, y);
    }

    pub fn finish_drawing(&mut self, x: u16, y: u16) {
        self.active_tool.on_mouse_up(x, y, &mut self.canvas);
    }

    pub fn cancel_drawing(&mut self) {
        self.active_tool.cancel();
    }

    pub fn is_drawing(&self) -> bool {
        self.active_tool.is_drawing()
    }

    pub fn get_preview_points(&self) -> Vec<(i32, i32, char)> {
        self.active_tool.preview_points()
    }

    pub fn is_text_input_mode(&self) -> bool {
        self.selected_tool == Tool::Text && self.is_drawing()
    }

    pub fn add_text_char(&mut self, c: char) {
        if let Some(text_tool) = self.active_tool.as_any_mut().downcast_mut::<TextTool>() {
            text_tool.add_char(c);
        }
    }

    pub fn text_backspace(&mut self) {
        if let Some(text_tool) = self.active_tool.as_any_mut().downcast_mut::<TextTool>() {
            text_tool.backspace();
        }
    }

    pub fn finish_text_input(&mut self) {
        // Trigger on_mouse_up to commit the text (we don't care about the position)
        self.active_tool.on_mouse_up(0, 0, &mut self.canvas);
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

        // Create new tool instance based on selection
        self.active_tool = match tool {
            Tool::Line => Box::new(LineTool::new()),
            Tool::Rectangle => Box::new(RectangleTool::new()),
            Tool::Arrow => Box::new(ArrowTool::new()),
            Tool::Text => Box::new(TextTool::new()),
            // TODO: Implement select tool
            _ => Box::new(LineTool::new()),
        };
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
