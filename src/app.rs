use crate::canvas::Canvas;
use crate::state::{CommandExecutor, CommandState, HelpState, SelectionState};
use crate::tools::{ArrowTool, DrawingTool, LineTool, RectangleTool, TextTool, Tool};
use crate::types::Panel;
use crate::ui::UILayout;

/// Main application state
pub struct App {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub active_panel: Panel,
    pub selected_tool: Tool,
    pub tool_index: usize, // For arrow key navigation
    pub tool_locked: bool, // If true, tool stays active after drawing
    pub layout: UILayout,
    pub help: HelpState,
    pub command: CommandState,
    // File state
    pub current_file: Option<String>, // Path to currently open file
    pub status_message: Option<String>, // Temporary message for statusbar
    // Drawing canvas
    pub canvas: Canvas,
    // Active tool instance (None when in Select mode)
    active_tool: Option<Box<dyn DrawingTool>>,
    // Selection state (for Select tool)
    pub selection_state: SelectionState,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            active_panel: Panel::Canvas,
            selected_tool: Tool::Select,
            tool_index: 0,
            tool_locked: false,
            layout: UILayout::default(),
            help: HelpState::new(),
            command: CommandState::new(),
            current_file: None,
            status_message: None,
            canvas: Canvas::default(),
            active_tool: None, // No active tool when in Select mode
            selection_state: SelectionState::new(),
        }
    }

    // ============================================================================
    // Command Mode
    // ============================================================================
    pub fn enter_command_mode(&mut self) {
        self.command.enter();
    }

    pub fn enter_command_mode_with(&mut self, command: &str) {
        self.command.enter_with(command);
    }

    pub fn exit_command_mode(&mut self) {
        self.command.exit();
        self.status_message = None; // Clear status message when manually exiting
    }

    pub fn is_command_mode_active(&self) -> bool {
        self.command.is_active()
    }

    pub fn add_char_to_command(&mut self, c: char) {
        self.command.add_char(c);
    }

    pub fn backspace_command(&mut self) {
        self.command.backspace();
    }

    pub fn execute_command(&mut self) {
        let action = self.command.parse();
        CommandExecutor::execute(action, self);
        self.command.finish();
    }

    // ============================================================================
    // Help Modal
    // ============================================================================

    pub fn toggle_help(&mut self) {
        self.help.toggle();
    }

    pub fn scroll_help_up(&mut self) {
        self.help.scroll_up();
    }

    pub fn scroll_help_down(&mut self) {
        self.help.scroll_down(&self.layout);
    }

    // ============================================================================
    // Tool Management & Drawing
    // ============================================================================

    pub fn toggle_tool_lock(&mut self) {
        self.tool_locked = !self.tool_locked;
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

    pub fn is_select_tool(&self) -> bool {
        self.selected_tool == Tool::Select
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

    pub fn finish_drawing(&mut self, x: u16, y: u16) -> bool {
        if let Some(tool) = &mut self.active_tool {
            let elements_before = self.canvas.elements().len();
            tool.on_mouse_up(x, y, &mut self.canvas);
            let elements_after = self.canvas.elements().len();
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

    pub fn finish_text_input(&mut self) -> bool {
        if let Some(tool) = &mut self.active_tool {
            let elements_before = self.canvas.elements().len();
            tool.on_mouse_up(0, 0, &mut self.canvas);
            let elements_after = self.canvas.elements().len();
            // Return true if an element was actually created
            elements_after > elements_before
        } else {
            false
        }
    }

    // ============================================================================
    // Selection Mode
    // ============================================================================

    /// Get selection box points for drag-selection visualization
    pub fn get_selection_box_points(&self) -> Vec<(i32, i32, char)> {
        self.selection_state.get_selection_box_points()
    }

    /// Get IDs of selected elements
    pub fn get_selected_element_ids(&self) -> &[usize] {
        self.selection_state.get_selected_element_ids()
    }

    /// Get move offset for selected elements during moving
    pub fn get_move_offset(&self) -> Option<(i32, i32)> {
        self.selection_state.get_move_offset()
    }

    pub fn is_in_selection_mode(&self) -> bool {
        self.selection_state.is_in_selection_mode()
    }

    // Selection operations

    pub fn start_selection(&mut self, x: u16, y: u16) {
        self.selection_state.start_selection(x, y);
    }

    pub fn update_selection(&mut self, x: u16, y: u16) {
        self.selection_state.update_selection(x, y);
    }

    pub fn finish_selection(&mut self, x: u16, y: u16) {
        self.selection_state.finish_selection(x, y, &self.canvas);
    }

    /// Toggle selection of element at position (for Shift+Click additive selection)
    pub fn toggle_selection_at(&mut self, x: i32, y: i32) {
        self.selection_state.toggle_selection_at(x, y, &self.canvas);
    }

    pub fn start_move_selection(&mut self, x: u16, y: u16) {
        self.selection_state
            .start_move_selection(x, y, &self.canvas);
    }

    pub fn update_move_selection(&mut self, x: u16, y: u16) {
        self.selection_state.update_move_selection(x, y);
    }

    pub fn finish_move_selection(&mut self) {
        self.selection_state.finish_move_selection(&mut self.canvas);
    }

    pub fn deselect(&mut self) {
        self.selection_state.deselect();
    }

    /// Move selected elements by offset (used for arrow key movement)
    pub fn move_selected_elements(&mut self, dx: i32, dy: i32) {
        self.selection_state
            .move_selected_elements(dx, dy, &mut self.canvas);
    }

    /// Delete selected elements
    pub fn delete_selected_elements(&mut self) {
        self.selection_state
            .delete_selected_elements(&mut self.canvas);
    }

    // ============================================================================
    // UI State
    // ============================================================================

    pub fn switch_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
    }

    pub fn update_cursor(&mut self, x: u16, y: u16) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    // ============================================================================
    // File I/O
    // ============================================================================

    /// Save the diagram to a file
    pub fn save_to_file(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.canvas.save_to_file(&path)?;
        self.current_file = Some(path.as_ref().display().to_string());
        self.status_message = Some(format!("Saved to {}", path.as_ref().display()));
        Ok(())
    }

    /// Load a diagram from a file
    pub fn load_from_file(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        self.load_from_file_with_message(&path, true)
    }

    /// Load a diagram from a file, optionally showing a status message
    fn load_from_file_with_message(
        &mut self,
        path: impl AsRef<std::path::Path>,
        show_message: bool,
    ) -> anyhow::Result<()> {
        self.canvas.load_from_file(&path)?;
        self.current_file = Some(path.as_ref().display().to_string());
        if show_message {
            self.status_message = Some(format!("Loaded from {}", path.as_ref().display()));
        }
        Ok(())
    }

    /// Load a diagram from a file silently (for initial load)
    pub fn load_from_file_silent(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<()> {
        self.load_from_file_with_message(&path, false)
    }

    /// Set a status message to display
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
    }

    /// Clear the status message
    pub fn clear_status_message(&mut self) {
        self.status_message = None;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
