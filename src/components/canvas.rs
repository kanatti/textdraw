use crate::components::Component;
use crate::events::{ActionType, EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::state::AppState;
use crate::tools::Tool;
use crate::types::{Panel, RenderMap, SelectionMode};
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::collections::HashMap;

pub struct CanvasComponent;

impl CanvasComponent {
    pub fn new() -> Self {
        Self
    }

    /// Generate a map of (x, y) -> (char, color) for welcome text
    fn get_welcome_text_map(
        &self,
        area: &ratatui::layout::Rect,
    ) -> HashMap<(usize, usize), (char, Color)> {
        let canvas_height = area.height.saturating_sub(2) as usize;
        let canvas_width = area.width.saturating_sub(2) as usize;

        let welcome_lines = vec![
            ("TextDraw v0.1.0", Color::Green),
            ("", Color::DarkGray),
            (
                "Interactive terminal ASCII diagram editor built with Ratatui.",
                Color::White,
            ),
            (
                "Create and edit diagrams using simple drawing tools and keyboard shortcuts.",
                Color::White,
            ),
            ("", Color::DarkGray),
            ("Press <space> for switching tools", Color::White),
            ("Press ? for more help", Color::White),
            ("", Color::DarkGray),
            ("Start drawing...", Color::DarkGray),
        ];

        let mut map = HashMap::new();
        let total_lines = welcome_lines.len();
        let start_y = canvas_height.saturating_sub(total_lines) / 2;

        for (idx, (text, color)) in welcome_lines.iter().enumerate() {
            let y = start_y + idx;
            let text_len = text.len();
            let start_x = canvas_width.saturating_sub(text_len) / 2;

            for (char_idx, ch) in text.chars().enumerate() {
                let x = start_x + char_idx;
                map.insert((x, y), (ch, *color));
            }
        }

        map
    }
}

/// Check if we should handle the event based on canvas bounds, drawing state, and active panel
fn should_handle_event(state: &AppState, mouse_event: &MouseEvent) -> bool {
    // Don't handle if canvas is not the active panel
    if state.active_panel != Panel::Canvas {
        return false;
    }

    // Handle if already drawing (even if outside canvas)
    if state.is_drawing() {
        return true;
    }

    // Otherwise only handle if inside canvas bounds
    state.is_inside_canvas(mouse_event.column, mouse_event.row)
}

impl EventHandler for CanvasComponent {
    type State = AppState;

    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Handle Edit Table mode first (highest priority)
        if state.is_editing_table() {
            return self.handle_edit_table_key(state, key_event);
        }

        // Forward to active tool first
        if let Some(tool) = state.tool.active_tool_mut() {
            let result = tool.handle_key_event(&mut state.canvas, key_event);
            match result {
                EventResult::Action(ActionType::FinishedDrawing) => {
                    if !state.tool.tool_locked {
                        state.select_tool(Tool::Select);
                    }
                    return EventResult::Consumed;
                }
                EventResult::Consumed => return EventResult::Consumed,
                EventResult::Ignored => {
                    // Continue to canvas-level handling
                }
                _ => return result,
            }
        }

        // Handle canvas-level selection operations (move/delete)
        if state.is_select_tool() && state.is_in_selection_mode() {
            match key_event.code {
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    let (dx, dy) = match key_event.code {
                        KeyCode::Up => (0, -1),
                        KeyCode::Down => (0, 1),
                        KeyCode::Left => (-1, 0),
                        KeyCode::Right => (1, 0),
                        _ => unreachable!(),
                    };
                    state.move_selected_elements(dx, dy);
                    return EventResult::Consumed;
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    state.delete_selected_elements();
                    return EventResult::Consumed;
                }
                KeyCode::Enter | KeyCode::Char('e') => {
                    // Enter Edit Table mode if a table is selected
                    return self.try_enter_edit_table_mode(state);
                }
                _ => {}
            }
        }

        EventResult::Ignored
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // If clicking inside canvas bounds and canvas is not active, just activate it
        if state.is_inside_canvas(mouse_event.column, mouse_event.row) {
            if state.active_panel != Panel::Canvas {
                state.switch_panel(Panel::Canvas);
                // Update cursor position but don't process drawing
                if let Some(canvas_event) = self.to_canvas_event(state, mouse_event) {
                    state.update_cursor(canvas_event.column, canvas_event.row);
                }
                return EventResult::Consumed;
            }
        }

        if !should_handle_event(state, mouse_event) {
            return EventResult::Ignored;
        }

        // Mark user action (hides welcome screen)
        state.mark_user_action();

        // Convert to canvas coordinates
        let Some(canvas_event) = self.to_canvas_event(state, mouse_event) else {
            // Outside canvas bounds - cancel drawing if active
            if state.is_drawing() {
                state.cancel_drawing();
            }
            return EventResult::Ignored;
        };

        // Forward to select tool or active drawing tool
        if state.is_select_tool() {
            self.handle_selection_mouse_down(
                state,
                canvas_event.column,
                canvas_event.row,
                mouse_event.is_shift(),
            );
            EventResult::Consumed
        } else if let Some(tool) = state.tool.active_tool_mut() {
            let result = tool.handle_mouse_down(&mut state.canvas, &canvas_event);
            match result {
                EventResult::Action(ActionType::FinishedDrawing) => {
                    if !state.tool.tool_locked {
                        state.select_tool(Tool::Select);
                    }
                    return EventResult::Consumed;
                }
                _ => result,
            }
        } else {
            EventResult::Ignored
        }
    }

    fn handle_mouse_up(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        if !should_handle_event(state, mouse_event) {
            return EventResult::Ignored;
        }

        // Convert to canvas coordinates
        let Some(canvas_event) = self.to_canvas_event(state, mouse_event) else {
            // Outside canvas bounds - cancel drawing if active
            if state.is_drawing() {
                state.cancel_drawing();
            }
            return EventResult::Ignored;
        };

        // Handle selection
        if state.is_select_tool() {
            match state.selection_state.mode {
                SelectionMode::Selecting => {
                    state.finish_selection(canvas_event.column, canvas_event.row);
                }
                SelectionMode::Moving => {
                    state.finish_move_selection();
                }
                _ => {}
            }
            return EventResult::Consumed;
        }

        // Forward to active drawing tool
        if let Some(tool) = state.tool.active_tool_mut() {
            let result = tool.handle_mouse_up(&mut state.canvas, &canvas_event);
            match result {
                EventResult::Action(ActionType::FinishedDrawing) => {
                    if !state.tool.tool_locked {
                        state.select_tool(Tool::Select);
                    }
                    return EventResult::Consumed;
                }
                _ => return result,
            }
        }

        EventResult::Ignored
    }

    fn handle_mouse_moved(
        &mut self,
        state: &mut AppState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        // Only handle if canvas is active
        if state.active_panel != Panel::Canvas {
            return EventResult::Ignored;
        }

        // Update cursor position
        if let Some(canvas_event) = self.to_canvas_event(state, mouse_event) {
            state.update_cursor(canvas_event.column, canvas_event.row);

            // Forward to active drawing tool if we're currently drawing
            if state.is_drawing() {
                if let Some(tool) = state.tool.active_tool_mut() {
                    return tool.handle_mouse_moved(&mut state.canvas, &canvas_event);
                }
            }
        }

        EventResult::Consumed
    }

    fn handle_mouse_drag(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        if !should_handle_event(state, mouse_event) {
            return EventResult::Ignored;
        }

        // Convert to canvas coordinates
        let Some(canvas_event) = self.to_canvas_event(state, mouse_event) else {
            // Outside canvas bounds - cancel drawing if active
            if state.is_drawing() {
                state.cancel_drawing();
            }
            return EventResult::Ignored;
        };

        state.update_cursor(canvas_event.column, canvas_event.row);

        // Handle selection
        if state.is_select_tool() {
            if state.is_in_selection_mode() {
                if state.selection_state.mode == SelectionMode::Selecting {
                    state.update_selection(canvas_event.column, canvas_event.row);
                } else if state.selection_state.mode == SelectionMode::Moving {
                    state.update_move_selection(canvas_event.column, canvas_event.row);
                }
            }
            return EventResult::Consumed;
        }

        // Forward to active drawing tool
        if let Some(tool) = state.tool.active_tool_mut() {
            return tool.handle_mouse_drag(&mut state.canvas, &canvas_event);
        }

        EventResult::Ignored
    }
}

impl CanvasComponent {
    /// Convert screen coordinates to canvas coordinates
    fn to_canvas_coords(&self, state: &AppState, column: u16, row: u16) -> Option<(u16, u16)> {
        let canvas_area = state.layout.canvas;
        // First check if click is within the canvas area at all
        if column < canvas_area.x
            || column >= canvas_area.x + canvas_area.width
            || row < canvas_area.y
            || row >= canvas_area.y + canvas_area.height
        {
            return None;
        }

        let canvas_x = column.saturating_sub(canvas_area.x + 1);
        let canvas_y = row.saturating_sub(canvas_area.y + 1);

        // Check if within canvas bounds (excluding borders)
        if canvas_x < canvas_area.width.saturating_sub(2)
            && canvas_y < canvas_area.height.saturating_sub(2)
        {
            return Some((canvas_x, canvas_y));
        }

        None
    }

    /// Convert a screen-space mouse event to canvas-space
    /// Returns None if the event is outside the canvas bounds
    fn to_canvas_event(&self, state: &AppState, mouse_event: &MouseEvent) -> Option<MouseEvent> {
        let (canvas_x, canvas_y) =
            self.to_canvas_coords(state, mouse_event.column, mouse_event.row)?;
        Some(mouse_event.with_coords(canvas_x, canvas_y))
    }

    /// Handle mouse down in selection mode
    fn handle_selection_mouse_down(
        &self,
        state: &mut AppState,
        canvas_x: u16,
        canvas_y: u16,
        shift_pressed: bool,
    ) {
        // Shift+Click: toggle selection at this position (additive selection)
        if shift_pressed {
            state.toggle_selection_at(canvas_x as i32, canvas_y as i32);
            return;
        }

        // Normal click behavior
        if state.is_in_selection_mode() {
            // Check if clicking inside any selected element's bounds
            let clicked_selected = self.is_clicking_selected_element(state, canvas_x, canvas_y);

            if clicked_selected {
                // Start moving selection
                state.start_move_selection(canvas_x, canvas_y);
            } else {
                // Clicked outside selected elements - deselect and start new selection
                state.deselect();
                state.start_selection(canvas_x, canvas_y);
            }
        } else {
            // No selection - start new selection
            state.start_selection(canvas_x, canvas_y);
        }
    }

    /// Check if click is inside any selected element
    fn is_clicking_selected_element(&self, state: &AppState, canvas_x: u16, canvas_y: u16) -> bool {
        let px = canvas_x as i32;
        let py = canvas_y as i32;

        for element_id in state.get_selected_element_ids() {
            if let Some(element) = state.canvas.get_element(*element_id) {
                if element.point_in_bounds(px, py) {
                    return true;
                }
            }
        }

        false
    }

    /// Build overlays for Edit Table mode (selected cell highlight + edit buffer content)
    fn build_edit_table_overlays(&self, state: &AppState) -> (HashMap<(i32, i32), bool>, HashMap<(i32, i32), (char, bool)>) {
        use crate::elements::{Element, TableElement};

        let mut highlight_map = HashMap::new();
        let mut content_map = HashMap::new();

        let Some(edit_state) = &state.editing_table else {
            return (highlight_map, content_map);
        };

        let Some(Element::Table(table)) = state.canvas.get_element(edit_state.table_id) else {
            return (highlight_map, content_map);
        };

        let x = table.start.x as i32;
        let y = table.start.y as i32;
        let row_height = TableElement::CELL_HEIGHT;

        let selected_row = edit_state.selected_row;
        let selected_col = edit_state.selected_col;

        // Calculate selected cell position using dynamic column widths
        let col_width = table.get_column_width(selected_col);
        let mut cell_x = x;
        for col in 0..selected_col {
            cell_x += table.get_column_width(col) as i32 + 1;
        }
        let cell_y = y + (selected_row as i32 * (row_height as i32 + 1));

        // Add highlight for content area only (inside borders)
        // Content area is from (cell_x + 1, cell_y + 1) to (cell_x + col_width, cell_y + row_height)
        for dy in 1..=row_height {
            for dx in 1..=col_width {
                highlight_map.insert((cell_x + dx as i32, cell_y + dy as i32), true);
            }
        }

        // If editing cell, fill the entire cell width with edit buffer content
        if edit_state.editing_cell {
            let content_x = cell_x + 1;
            let content_y = cell_y + 1;

            // Add edit buffer text with cursor
            let mut buffer_with_cursor = edit_state.edit_buffer.clone();
            buffer_with_cursor.insert(edit_state.cursor_pos, '│');

            // Fill entire cell width - pad with spaces to cover original text
            for i in 0..col_width {
                let ch = buffer_with_cursor.chars().nth(i as usize).unwrap_or(' ');
                let is_cursor = i as usize == edit_state.cursor_pos;
                content_map.insert((content_x + i as i32, content_y), (ch, is_cursor));
            }
        }

        (highlight_map, content_map)
    }

    /// Try to enter Edit Table mode if a single table is selected
    fn try_enter_edit_table_mode(&self, state: &mut AppState) -> EventResult {
        let selected_ids = state.get_selected_element_ids();
        if selected_ids.len() == 1 {
            let element_id = selected_ids[0];
            if let Some(element) = state.canvas.get_element(element_id) {
                // Check if it's a table
                if matches!(element, crate::elements::Element::Table(_)) {
                    state.enter_edit_table_mode(element_id);
                    return EventResult::Consumed;
                }
            }
        }
        EventResult::Ignored
    }

    /// Handle keyboard events in Edit Table mode
    fn handle_edit_table_key(&self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        use crate::elements::Element;

        // Get table_id and editing_cell status first
        let (table_id, editing_cell) = {
            let Some(edit_state) = state.editing_table.as_ref() else {
                return EventResult::Ignored;
            };
            (edit_state.table_id, edit_state.editing_cell)
        };

        // Check if table still exists
        if state.canvas.get_element(table_id).is_none() {
            state.exit_edit_table_mode();
            return EventResult::Consumed;
        }

        // If editing a cell, handle text input
        if editing_cell {
            match key_event.code {
                KeyCode::Esc => {
                    // Cancel editing, restore original content
                    let (row, col, original) = {
                        let edit_state = state.editing_table_mut().unwrap();
                        let row = edit_state.selected_row;
                        let col = edit_state.selected_col;
                        let original = edit_state.original_content.clone();
                        edit_state.editing_cell = false;
                        edit_state.edit_buffer.clear();
                        (row, col, original)
                    };

                    // Restore original content
                    let Some(Element::Table(table)) = state.canvas.get_element_mut(table_id) else {
                        return EventResult::Consumed;
                    };
                    if row < table.cells.len() && col < table.cells[row].len() {
                        table.cells[row][col] = original;
                        table.update_size_from_content();
                    }

                    return EventResult::Consumed;
                }
                KeyCode::Enter => {
                    // Finish editing (content already saved in real-time)
                    let edit_state = state.editing_table_mut().unwrap();
                    edit_state.editing_cell = false;
                    edit_state.edit_buffer.clear();
                    return EventResult::Consumed;
                }
                KeyCode::Char(c) => {
                    let (row, col, new_content) = {
                        let edit_state = state.editing_table_mut().unwrap();
                        edit_state.edit_buffer.insert(edit_state.cursor_pos, c);
                        edit_state.cursor_pos += 1;
                        (edit_state.selected_row, edit_state.selected_col, edit_state.edit_buffer.clone())
                    };

                    // Update table cell and resize
                    let Some(Element::Table(table)) = state.canvas.get_element_mut(table_id) else {
                        return EventResult::Consumed;
                    };
                    if row < table.cells.len() && col < table.cells[row].len() {
                        table.cells[row][col] = new_content;
                        table.update_size_from_content();
                    }

                    return EventResult::Consumed;
                }
                KeyCode::Backspace => {
                    let (row, col, new_content) = {
                        let edit_state = state.editing_table_mut().unwrap();
                        if edit_state.cursor_pos > 0 {
                            edit_state.cursor_pos -= 1;
                            edit_state.edit_buffer.remove(edit_state.cursor_pos);
                        }
                        (edit_state.selected_row, edit_state.selected_col, edit_state.edit_buffer.clone())
                    };

                    // Update table cell and resize
                    let Some(Element::Table(table)) = state.canvas.get_element_mut(table_id) else {
                        return EventResult::Consumed;
                    };
                    if row < table.cells.len() && col < table.cells[row].len() {
                        table.cells[row][col] = new_content;
                        table.update_size_from_content();
                    }

                    return EventResult::Consumed;
                }
                KeyCode::Delete => {
                    let (row, col, new_content) = {
                        let edit_state = state.editing_table_mut().unwrap();
                        if edit_state.cursor_pos < edit_state.edit_buffer.len() {
                            edit_state.edit_buffer.remove(edit_state.cursor_pos);
                        }
                        (edit_state.selected_row, edit_state.selected_col, edit_state.edit_buffer.clone())
                    };

                    // Update table cell and resize
                    let Some(Element::Table(table)) = state.canvas.get_element_mut(table_id) else {
                        return EventResult::Consumed;
                    };
                    if row < table.cells.len() && col < table.cells[row].len() {
                        table.cells[row][col] = new_content;
                        table.update_size_from_content();
                    }

                    return EventResult::Consumed;
                }
                KeyCode::Left => {
                    let edit_state = state.editing_table_mut().unwrap();
                    if edit_state.cursor_pos > 0 {
                        edit_state.cursor_pos -= 1;
                    }
                    return EventResult::Consumed;
                }
                KeyCode::Right => {
                    let edit_state = state.editing_table_mut().unwrap();
                    if edit_state.cursor_pos < edit_state.edit_buffer.len() {
                        edit_state.cursor_pos += 1;
                    }
                    return EventResult::Consumed;
                }
                _ => return EventResult::Consumed,
            }
        }

        // Not editing cell - handle navigation and cell editing activation
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                // Exit Edit Table mode
                state.exit_edit_table_mode();
                EventResult::Consumed
            }
            KeyCode::Up => {
                let edit_state = state.editing_table_mut().unwrap();
                if edit_state.selected_row > 0 {
                    edit_state.selected_row -= 1;
                }
                EventResult::Consumed
            }
            KeyCode::Down => {
                let max_rows = {
                    let Some(Element::Table(table)) = state.canvas.get_element(table_id) else {
                        return EventResult::Consumed;
                    };
                    table.rows
                };
                let edit_state = state.editing_table_mut().unwrap();
                if edit_state.selected_row < max_rows - 1 {
                    edit_state.selected_row += 1;
                }
                EventResult::Consumed
            }
            KeyCode::Left => {
                let edit_state = state.editing_table_mut().unwrap();
                if edit_state.selected_col > 0 {
                    edit_state.selected_col -= 1;
                }
                EventResult::Consumed
            }
            KeyCode::Right => {
                let max_cols = {
                    let Some(Element::Table(table)) = state.canvas.get_element(table_id) else {
                        return EventResult::Consumed;
                    };
                    table.cols
                };
                let edit_state = state.editing_table_mut().unwrap();
                if edit_state.selected_col < max_cols - 1 {
                    edit_state.selected_col += 1;
                }
                EventResult::Consumed
            }
            KeyCode::Enter => {
                // Start editing current cell - load its content into edit buffer
                let cell_content = {
                    let Some(Element::Table(table)) = state.canvas.get_element(table_id) else {
                        return EventResult::Consumed;
                    };

                    let edit_state = state.editing_table.as_ref().unwrap();
                    let row = edit_state.selected_row;
                    let col = edit_state.selected_col;

                    if row < table.cells.len() && col < table.cells[row].len() {
                        table.cells[row][col].clone()
                    } else {
                        String::new()
                    }
                };

                let edit_state = state.editing_table_mut().unwrap();
                edit_state.edit_buffer = cell_content.clone();
                edit_state.original_content = cell_content; // Save for cancel (Esc)
                edit_state.cursor_pos = edit_state.edit_buffer.len();
                edit_state.editing_cell = true;

                EventResult::Consumed
            }
            _ => EventResult::Consumed,
        }
    }
}

impl Component for CanvasComponent {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        let area = state.layout.canvas;
        let mut lines = vec![];

        // Get preview points from the active tool
        let preview_points = state.get_preview_points();
        let preview_map: RenderMap = preview_points
            .into_iter()
            .map(|(x, y, ch)| ((x, y), ch))
            .collect();

        // Get selection box (grey) for drag-select
        let selection_box = state.get_selection_box_points();
        let selection_box_map: RenderMap = selection_box
            .into_iter()
            .map(|(x, y, ch)| ((x, y), ch))
            .collect();

        // Get selected element IDs and move offset
        let selected_ids = state.get_selected_element_ids();
        let move_offset = state.get_move_offset();

        // Build render cache: map (x, y) -> (char, element_id)
        // This is O(total_points) instead of O(pixels × elements)
        let mut render_map: HashMap<(i32, i32), (char, usize)> = HashMap::new();

        for element in state.canvas.elements() {
            let element_id = element.id();
            let is_selected = selected_ids.contains(&element_id);

            // Calculate offset for selected elements being moved
            let (offset_x, offset_y) = if is_selected {
                move_offset.unwrap_or((0, 0))
            } else {
                (0, 0)
            };

            // Generate points from element and add to render map (with offset if moving)
            let points = element.render_points();
            for (x, y, ch) in points {
                let render_x = x + offset_x;
                let render_y = y + offset_y;
                render_map.insert((render_x, render_y), (ch, element_id));
            }
        }

        // Build Edit Table mode overlays (selected cell highlight + edit buffer)
        let (edit_table_highlight_map, edit_table_content_map) = self.build_edit_table_overlays(state);

        // Check if actively editing a cell (for color selection)
        let is_actively_editing_cell = state.editing_table.as_ref().map(|e| e.editing_cell).unwrap_or(false);

        // Calculate welcome text positioning if needed
        let welcome_text_map = if state.should_show_welcome() {
            Some(self.get_welcome_text_map(&area))
        } else {
            None
        };

        for y in 0..area.height.saturating_sub(2) {
            let mut line_chars = vec![];
            for x in 0..area.width.saturating_sub(2) {
                let px = x as i32;
                let py = y as i32;

                // Priority: cursor > welcome text > selection box > preview > elements

                // Check if actively selecting or moving
                let is_actively_selecting_or_moving = matches!(
                    state.selection_state.mode,
                    SelectionMode::Selecting | SelectionMode::Moving
                );

                // Check if hovering over a selected element (to hide cursor)
                let mut hovering_selected = false;
                if state.is_select_tool() && !selected_ids.is_empty() {
                    for element_id in selected_ids {
                        if let Some(element) = state.canvas.get_element(*element_id) {
                            if element.point_in_bounds(px, py) {
                                hovering_selected = true;
                                break;
                            }
                        }
                    }
                }

                if x == state.cursor_x
                    && y == state.cursor_y
                    && state.active_panel == Panel::Canvas
                    && !state.is_drawing()
                    && !is_actively_selecting_or_moving
                    && !hovering_selected
                {
                    // Show cursor block only when:
                    // - Canvas is active
                    // - Not drawing
                    // - Not actively selecting/moving
                    // - Not hovering over a selected element
                    line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
                } else if let Some(ref map) = welcome_text_map {
                    if let Some((ch, color)) = map.get(&(x as usize, y as usize)) {
                        // Show welcome text - grey if canvas not active
                        let text_color = if state.active_panel == Panel::Canvas {
                            *color
                        } else {
                            Color::DarkGray
                        };
                        line_chars.push(Span::styled(ch.to_string(), Style::default().fg(text_color)));
                    } else {
                        line_chars.push(Span::raw(" "));
                    }
                } else if let Some(&ch) = selection_box_map.get(&(px, py)) {
                    // Show selection box in grey (during drag-select)
                    line_chars.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(Color::DarkGray),
                    ));
                } else if let Some(&ch) = preview_map.get(&(px, py)) {
                    // Show preview while drawing
                    let preview_color = Color::DarkGray;
                    line_chars.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(preview_color),
                    ));
                } else if let Some((edit_ch, is_cursor)) = edit_table_content_map.get(&(px, py)) {
                    // Edit Table mode: show edit buffer content with cursor
                    // Blue background when editing, cursor is shown as │ character
                    let style = Style::default().bg(Color::Blue).fg(Color::White);
                    line_chars.push(Span::styled(edit_ch.to_string(), style));
                } else if let Some((ch, element_id)) = render_map.get(&(px, py)) {
                    // Found element at this position - O(1) lookup!
                    let is_selected = selected_ids.contains(element_id);

                    // Check if this position is in the highlighted cell (Edit Table mode)
                    let in_highlighted_cell = edit_table_highlight_map.contains_key(&(px, py));

                    // Use grey color when canvas is not active
                    let mut style = if state.active_panel != Panel::Canvas {
                        Style::default().fg(Color::DarkGray) // Grey when canvas not active
                    } else if is_selected {
                        Style::default().fg(Color::Yellow) // Selected elements in yellow
                    } else {
                        Style::default().fg(Color::White) // Normal elements in white
                    };

                    // Add background for highlighted cell: DarkGray when selecting, Blue when editing
                    if in_highlighted_cell {
                        let bg_color = if is_actively_editing_cell {
                            Color::Blue
                        } else {
                            Color::DarkGray
                        };
                        style = style.bg(bg_color);
                    }

                    line_chars.push(Span::styled(ch.to_string(), style));
                } else if edit_table_highlight_map.contains_key(&(px, py)) {
                    // Empty space inside highlighted cell - DarkGray when selecting, Blue when editing
                    let bg_color = if is_actively_editing_cell {
                        Color::Blue
                    } else {
                        Color::DarkGray
                    };
                    line_chars.push(Span::styled(" ", Style::default().bg(bg_color)));
                } else {
                    // Empty space
                    line_chars.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(line_chars));
        }

        let canvas_style = if !state.show_help && state.active_panel == Panel::Canvas {
            Style::default().fg(Color::Green)
        } else if state.active_panel != Panel::Canvas {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        // Build title with filename if available
        let title = if let Some(ref file) = state.file.current_file {
            // Extract just the filename from the path
            let filename = std::path::Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file);
            format!("Canvas ─── {} ───", filename)
        } else {
            "Canvas".to_string()
        };

        let canvas = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(title)
                .border_style(canvas_style),
        );

        frame.render_widget(canvas, area);
    }
}
