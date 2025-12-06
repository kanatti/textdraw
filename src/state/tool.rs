use crate::state::CanvasState;
use crate::tools::{ArrowTool, DrawingTool, LineTool, RectangleTool, TextTool, Tool};

pub struct ToolState {
    pub selected_tool: Tool,
    pub tool_index: usize, // For arrow key navigation
    pub tool_locked: bool, // If true, tool stays active after drawing
    active_tool: Option<Box<dyn DrawingTool>>,
}

impl ToolState {
    pub fn new() -> Self {
        Self {
            selected_tool: Tool::Select,
            tool_index: 0,
            tool_locked: false,
            active_tool: None, // No active tool when in Select mode
        }
    }

    // Tool selection

    pub fn select_tool(&mut self, tool: Tool) -> bool {
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

        // Return true if we switched away from Select tool (caller should deselect)
        tool != Tool::Select
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

    pub fn is_select_tool(&self) -> bool {
        self.selected_tool == Tool::Select
    }

    pub fn toggle_tool_lock(&mut self) {
        self.tool_locked = !self.tool_locked;
    }

    // Drawing tool operations

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

    pub fn finish_drawing(&mut self, x: u16, y: u16, canvas: &mut CanvasState) -> bool {
        if let Some(tool) = &mut self.active_tool {
            let elements_before = canvas.elements().len();
            tool.on_mouse_up(x, y, canvas);
            let elements_after = canvas.elements().len();
            // Return true if an element was actually created
            elements_after > elements_before
        } else {
            false
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

    // Text tool operations

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

    pub fn finish_text_input(&mut self, canvas: &mut CanvasState) -> bool {
        if let Some(tool) = &mut self.active_tool {
            let elements_before = canvas.elements().len();
            tool.on_mouse_up(0, 0, canvas);
            let elements_after = canvas.elements().len();
            // Return true if an element was actually created
            elements_after > elements_before
        } else {
            false
        }
    }
}
