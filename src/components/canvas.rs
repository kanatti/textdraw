use crate::state::AppState;
use crate::components::Component;
use crate::events::{EventHandler, EventResult};
use crate::tools::Tool;
use crate::types::{Panel, SelectionMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct CanvasComponent;

impl CanvasComponent {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for CanvasComponent {
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Handle text input when text tool is active and in drawing mode
        if state.is_text_input_mode() {
            return match key_event.code {
                KeyCode::Char(c) => {
                    state.add_text_char(c);
                    EventResult::Consumed
                }
                KeyCode::Backspace => {
                    state.text_backspace();
                    EventResult::Consumed
                }
                KeyCode::Enter | KeyCode::Esc => {
                    // Commit or cancel text
                    if key_event.code == KeyCode::Enter {
                        let element_created = state.finish_text_input();

                        // Switch to Select tool if not locked AND an element was actually created
                        if !state.tool.tool_locked && element_created {
                            state.select_tool(Tool::Select);
                        }
                    } else {
                        state.cancel_drawing();
                    }
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            };
        }

        // Handle selection operations (move/delete)
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
                _ => {}
            }
        }

        EventResult::Ignored
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // If text tool is active and we're in text input mode, finish the text
        if state.is_text_input_mode() {
            let element_created = state.finish_text_input();

            // Switch to Select tool if not locked AND an element was actually created
            if !state.tool.tool_locked && element_created {
                state.select_tool(Tool::Select);
            }
        }

        // Only handle if canvas is active
        if state.active_panel != Panel::Canvas {
            return EventResult::Ignored;
        }

        // Convert to canvas coordinates
        let canvas_coords = self.to_canvas_coords(state, mouse_event.column, mouse_event.row);
        let Some((canvas_x, canvas_y)) = canvas_coords else {
            return EventResult::Ignored;
        };

        // Handle based on tool
        if state.is_select_tool() {
            let shift_pressed = mouse_event.modifiers.contains(KeyModifiers::SHIFT);
            self.handle_selection_mouse_down(state, canvas_x, canvas_y, shift_pressed);
        } else {
            state.start_drawing(canvas_x, canvas_y);
        }

        EventResult::Consumed
    }

    fn handle_mouse_up(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Only handle if canvas is active
        if state.active_panel != Panel::Canvas {
            return EventResult::Ignored;
        }

        let Some((canvas_x, canvas_y)) =
            self.to_canvas_coords(state, mouse_event.column, mouse_event.row)
        else {
            return EventResult::Ignored;
        };

        if state.is_select_tool() {
            // Selection mode: finish selection or move
            match state.selection_state.mode {
                SelectionMode::Selecting => {
                    state.finish_selection(canvas_x, canvas_y);
                }
                SelectionMode::Moving => {
                    state.finish_move_selection();
                }
                _ => {}
            }
        } else {
            // Finish drawing on mouse up (except for text tool which finishes on Enter)
            if state.is_drawing() && !state.is_text_input_mode() {
                let element_created = state.finish_drawing(canvas_x, canvas_y);

                // Switch to Select tool if not locked AND an element was actually created
                if !state.tool.tool_locked && element_created {
                    state.select_tool(Tool::Select);
                }
            }
        }

        EventResult::Consumed
    }

    fn handle_mouse_moved(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Only handle if canvas is active
        if state.active_panel != Panel::Canvas {
            return EventResult::Ignored;
        }

        // Update cursor position
        if let Some((canvas_x, canvas_y)) =
            self.to_canvas_coords(state, mouse_event.column, mouse_event.row)
        {
            state.update_cursor(canvas_x, canvas_y);
        }

        EventResult::Consumed
    }

    fn handle_mouse_drag(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Only handle if canvas is active
        if state.active_panel != Panel::Canvas {
            return EventResult::Ignored;
        }

        let Some((canvas_x, canvas_y)) =
            self.to_canvas_coords(state, mouse_event.column, mouse_event.row)
        else {
            return EventResult::Ignored;
        };

        state.update_cursor(canvas_x, canvas_y);

        if state.is_select_tool() {
            // Selection mode: update selection or move
            if state.is_in_selection_mode() {
                if state.selection_state.mode == SelectionMode::Selecting {
                    state.update_selection(canvas_x, canvas_y);
                } else if state.selection_state.mode == SelectionMode::Moving {
                    state.update_move_selection(canvas_x, canvas_y);
                }
            }
        } else {
            // Handle dragging for drawing preview
            if state.is_drawing() {
                state.update_drawing(canvas_x, canvas_y);
            }
        }

        EventResult::Consumed
    }
}

impl CanvasComponent {
    /// Convert screen coordinates to canvas coordinates
    fn to_canvas_coords(&self, state: &AppState, column: u16, row: u16) -> Option<(u16, u16)> {
        if let Some(canvas_area) = state.layout.canvas {
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
        }
        None
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
}

impl Component for CanvasComponent {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        let Some(area) = state.layout.canvas else {
            return;
        };
        let mut lines = vec![];

        // Get preview points from the active tool
        let preview_points = state.get_preview_points();
        let preview_map: std::collections::HashMap<(i32, i32), char> = preview_points
            .into_iter()
            .map(|(x, y, ch)| ((x, y), ch))
            .collect();

        // Get selection box (grey) for drag-select
        let selection_box = state.get_selection_box_points();
        let selection_box_map: std::collections::HashMap<(i32, i32), char> = selection_box
            .into_iter()
            .map(|(x, y, ch)| ((x, y), ch))
            .collect();

        // Get selected element IDs and move offset
        let selected_ids = state.get_selected_element_ids();
        let move_offset = state.get_move_offset();

        // Build render cache: map (x, y) -> (char, element_id)
        // This is O(total_points) instead of O(pixels × elements)
        let mut render_map: std::collections::HashMap<(i32, i32), (char, usize)> =
            std::collections::HashMap::new();

        for element in state.canvas.elements() {
            let element_id = element.id();
            let is_selected = selected_ids.contains(&element_id);

            // Calculate offset for selected elements being moved
            let (offset_x, offset_y) = if is_selected {
                move_offset.unwrap_or((0, 0))
            } else {
                (0, 0)
            };

            // Add all points from this element to render map (with offset if moving)
            for ((x, y), ch) in element.points() {
                let render_x = x + offset_x;
                let render_y = y + offset_y;
                render_map.insert((render_x, render_y), (*ch, element_id));
            }
        }

        for y in 0..area.height.saturating_sub(2) {
            let mut line_chars = vec![];
            for x in 0..area.width.saturating_sub(2) {
                let px = x as i32;
                let py = y as i32;

                // Priority: cursor > selection box > preview > elements

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
                    && !state.is_drawing()
                    && !is_actively_selecting_or_moving
                    && !hovering_selected
                {
                    // Show cursor block only when:
                    // - Not drawing
                    // - Not actively selecting/moving
                    // - Not hovering over a selected element
                    line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
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
                } else if let Some((ch, element_id)) = render_map.get(&(px, py)) {
                    // Found element at this position - O(1) lookup!
                    let is_selected = selected_ids.contains(element_id);
                    let color = if is_selected {
                        Color::Yellow // Selected elements in yellow
                    } else {
                        Color::White // Normal elements in white
                    };
                    line_chars.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else {
                    // Empty space
                    line_chars.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(line_chars));
        }

        let canvas_style = if !state.show_help && state.active_panel == Panel::Canvas {
            Style::default().fg(Color::Green)
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
            format!("[0]-Canvas --- {} ---", filename)
        } else {
            "[0]-Canvas".to_string()
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
