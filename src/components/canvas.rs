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

pub struct CanvasComponent;

impl CanvasComponent {
    pub fn new() -> Self {
        Self
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
                _ => {}
            }
        }

        EventResult::Ignored
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
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

            // Generate points from element and add to render map (with offset if moving)
            let points = element.render_points();
            for (x, y, ch) in points {
                let render_x = x + offset_x;
                let render_y = y + offset_y;
                render_map.insert((render_x, render_y), (ch, element_id));
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
