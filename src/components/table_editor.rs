use crate::elements::{Element, TableElement};
use crate::events::{EventResult, KeyEvent};
use crate::state::AppState;
use crossterm::event::KeyCode;
use std::collections::HashMap;

// Type aliases for clarity
pub type HighlightMap = HashMap<(i32, i32), bool>;
pub type ContentMap = HashMap<(i32, i32), (char, bool)>;
pub type OverlayMaps = (HighlightMap, ContentMap);

/// Build overlays for Edit Table mode (selected cell highlight + edit buffer content)
pub fn build_edit_table_overlays(state: &AppState) -> OverlayMaps {
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
pub fn try_enter_edit_table_mode(state: &mut AppState) -> EventResult {
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
pub fn handle_edit_table_key(state: &mut AppState, key_event: &KeyEvent) -> EventResult {
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
                    (
                        edit_state.selected_row,
                        edit_state.selected_col,
                        edit_state.edit_buffer.clone(),
                    )
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
                    (
                        edit_state.selected_row,
                        edit_state.selected_col,
                        edit_state.edit_buffer.clone(),
                    )
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
                    (
                        edit_state.selected_row,
                        edit_state.selected_col,
                        edit_state.edit_buffer.clone(),
                    )
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
