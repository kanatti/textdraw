use crate::state::{
    CanvasState, CommandExecutor, CommandState, FileState, HelpState, SelectionState, ToolState,
};
use crate::types::Panel;
use crate::ui::UILayout;
use std::path::Path;

/// Main application state
pub struct AppState {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub active_panel: Panel,
    pub layout: UILayout,
    pub help: HelpState,
    pub command: CommandState,
    pub tool: ToolState,
    pub file: FileState,
    // Drawing canvas
    pub canvas: CanvasState,
    // Selection state (for Select tool)
    pub selection_state: SelectionState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            active_panel: Panel::Canvas,
            layout: UILayout::default(),
            help: HelpState::new(),
            command: CommandState::new(),
            tool: ToolState::new(),
            file: FileState::new(),
            canvas: CanvasState::default(),
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
        self.file.clear_status_message(); // Clear status message when manually exiting
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
    // Tool Management & Drawing
    // ============================================================================

    pub fn toggle_tool_lock(&mut self) {
        self.tool.toggle_tool_lock();
    }

    pub fn select_tool(&mut self, tool: crate::tools::Tool) {
        let should_deselect = self.tool.select_tool(tool);
        // Deselect when switching away from Select tool
        if should_deselect && self.is_in_selection_mode() {
            self.deselect();
        }
    }

    pub fn select_next_tool(&mut self) {
        self.tool.select_next_tool();
    }

    pub fn select_prev_tool(&mut self) {
        self.tool.select_prev_tool();
    }

    pub fn is_select_tool(&self) -> bool {
        self.tool.is_select_tool()
    }

    // Drawing tool operations

    pub fn start_drawing(&mut self, x: u16, y: u16) {
        self.tool.start_drawing(x, y);
    }

    pub fn update_drawing(&mut self, x: u16, y: u16) {
        self.tool.update_drawing(x, y);
    }

    pub fn finish_drawing(&mut self, x: u16, y: u16) -> bool {
        self.tool.finish_drawing(x, y, &mut self.canvas)
    }

    pub fn cancel_drawing(&mut self) {
        self.tool.cancel_drawing();
    }

    pub fn is_drawing(&self) -> bool {
        self.tool.is_drawing()
    }

    pub fn get_preview_points(&self) -> Vec<(i32, i32, char)> {
        self.tool.get_preview_points()
    }

    pub fn is_text_input_mode(&self) -> bool {
        self.tool.is_text_input_mode()
    }

    // Text tool operations

    pub fn add_text_char(&mut self, c: char) {
        self.tool.add_text_char(c);
    }

    pub fn text_backspace(&mut self) {
        self.tool.text_backspace();
    }

    pub fn finish_text_input(&mut self) -> bool {
        self.tool.finish_text_input(&mut self.canvas)
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

    /// Load a diagram from a file silently (for initial load)
    pub fn load_from_file_silent(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.file.load_from_file_silent(&mut self.canvas, path)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
