mod canvas;
mod command;
mod file;
mod selection;
mod tool;

pub use canvas::CanvasState;
pub use command::{CommandExecutor, CommandState};
pub use file::FileState;
pub use selection::SelectionState;
pub use tool::ToolState;

use crate::types::Panel;
use crate::ui::UILayout;
use std::path::Path;

/// Edit Table mode state
#[derive(Debug, Clone)]
pub struct EditTableState {
    pub table_id: usize,
    pub selected_row: usize,
    pub selected_col: usize,
    pub editing_cell: bool,
    pub edit_buffer: String,
    pub cursor_pos: usize,
    pub original_content: String, // Store original content for cancel (Esc)
}

/// Main application state
pub struct AppState {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub active_panel: Panel,
    pub layout: UILayout,
    pub show_help: bool,
    pub show_properties: bool,
    pub show_tools_modal: bool,
    pub command: CommandState,
    pub tool: ToolState,
    pub file: FileState,
    // Drawing canvas
    pub canvas: CanvasState,
    // Selection state (for Select tool)
    pub selection_state: SelectionState,
    // Edit Table mode state
    pub editing_table: Option<EditTableState>,
    // Track if user has taken any action (for welcome screen)
    pub has_user_action: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            active_panel: Panel::Canvas,
            layout: UILayout::default(),
            show_help: false,
            show_properties: true, // Default to showing properties
            show_tools_modal: false,
            command: CommandState::new(),
            tool: ToolState::new(),
            file: FileState::new(),
            canvas: CanvasState::default(),
            selection_state: SelectionState::new(),
            editing_table: None,
            has_user_action: false,
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
    // Help Modal & Properties Panel
    // ============================================================================
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_properties(&mut self) {
        self.show_properties = !self.show_properties;
    }

    pub fn toggle_tools_modal(&mut self) {
        self.show_tools_modal = !self.show_tools_modal;
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

    pub fn cancel_drawing(&mut self) {
        self.tool.cancel_drawing();
    }

    pub fn is_drawing(&self) -> bool {
        self.tool.is_drawing()
    }

    pub fn get_preview_points(&self) -> Vec<(i32, i32, char)> {
        self.tool.get_preview_points()
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
        // Exit edit mode if table is no longer selected
        self.check_edit_table_selection();
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
        // Exit edit table mode when deselecting
        if self.is_editing_table() {
            self.exit_edit_table_mode();
        }
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

    /// Check if screen coordinates are inside the canvas bounds
    pub fn is_inside_canvas(&self, column: u16, row: u16) -> bool {
        let canvas_area = self.layout.canvas;
        column >= canvas_area.x
            && column < canvas_area.x + canvas_area.width
            && row >= canvas_area.y
            && row < canvas_area.y + canvas_area.height
    }

    // ============================================================================
    // File I/O
    // ============================================================================

    /// Load a diagram from a file silently (for initial load)
    pub fn load_from_file_silent(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.file.load_from_file_silent(&mut self.canvas, path)
    }

    // ============================================================================
    // Welcome Screen
    // ============================================================================

    /// Mark that the user has taken an action (hides welcome screen)
    pub fn mark_user_action(&mut self) {
        self.has_user_action = true;
    }

    /// Check if the welcome screen should be shown
    pub fn should_show_welcome(&self) -> bool {
        !self.has_user_action && self.canvas.is_empty()
    }

    // ============================================================================
    // Edit Table Mode
    // ============================================================================

    /// Enter Edit Table mode for the given table
    pub fn enter_edit_table_mode(&mut self, table_id: usize) {
        self.editing_table = Some(EditTableState {
            table_id,
            selected_row: 0,
            selected_col: 0,
            editing_cell: false,
            edit_buffer: String::new(),
            cursor_pos: 0,
            original_content: String::new(),
        });
    }

    /// Exit Edit Table mode
    pub fn exit_edit_table_mode(&mut self) {
        self.editing_table = None;
    }

    /// Check if currently in Edit Table mode
    pub fn is_editing_table(&self) -> bool {
        self.editing_table.is_some()
    }

    /// Get mutable reference to Edit Table state if active
    pub fn editing_table_mut(&mut self) -> Option<&mut EditTableState> {
        self.editing_table.as_mut()
    }

    /// Check if the table being edited is still selected, exit edit mode if not
    pub fn check_edit_table_selection(&mut self) {
        if let Some(edit_state) = &self.editing_table {
            let table_id = edit_state.table_id;
            // Check if the table is still selected
            if !self.selection_state.selected_ids.contains(&table_id) {
                self.exit_edit_table_mode();
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
