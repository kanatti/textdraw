use crate::app::{App, Panel};
use crossterm::event::MouseEventKind;

pub fn handle_mouse_event(app: &mut App, kind: MouseEventKind, column: u16, row: u16) {
    match kind {
        MouseEventKind::Down(_) => {
            // If text tool is active and we're in text input mode, finish the text
            if app.is_text_input_mode() {
                app.finish_text_input();
            }

            // Check for tool click first
            if let Some(tool) = app.detect_tool_click(column, row) {
                app.select_tool(tool);
                app.switch_panel(Panel::Tools);
                return;
            }

            // Handle panel click
            if let Some(panel) = app.detect_panel_click(column, row) {
                app.switch_panel(panel);

                // If clicking on canvas
                if panel == Panel::Canvas {
                    if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                        if app.is_select_tool() {
                            // Selection mode
                            if app.is_in_selection_mode() {
                                // Check if clicking inside any selected element's bounds
                                let px = canvas_x as i32;
                                let py = canvas_y as i32;
                                let mut clicked_selected = false;

                                for element_id in app.get_selected_element_ids() {
                                    if let Some(element) = app.canvas.get_element(*element_id) {
                                        if element.point_in_bounds(px, py) {
                                            clicked_selected = true;
                                            break;
                                        }
                                    }
                                }

                                if clicked_selected {
                                    // Start moving selection
                                    app.start_move_selection(canvas_x, canvas_y);
                                } else {
                                    // Clicked outside selected elements - deselect and start new selection
                                    app.deselect();
                                    app.start_selection(canvas_x, canvas_y);
                                }
                            } else {
                                // No selection - start new selection
                                app.start_selection(canvas_x, canvas_y);
                            }
                        } else {
                            // Drawing tools: start drawing
                            app.start_drawing(canvas_x, canvas_y);
                        }
                    }
                }
            }
        }
        MouseEventKind::Up(_) => {
            if app.active_panel == Panel::Canvas {
                if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                    if app.is_select_tool() {
                        // Selection mode: finish selection or move
                        use crate::app::SelectionMode;
                        match app.selection_state.mode {
                            SelectionMode::Selecting => {
                                app.finish_selection(canvas_x, canvas_y);
                            }
                            SelectionMode::Moving => {
                                app.finish_move_selection();
                            }
                            _ => {}
                        }
                    } else {
                        // Finish drawing on mouse up (except for text tool which finishes on Enter)
                        if app.is_drawing() && !app.is_text_input_mode() {
                            app.finish_drawing(canvas_x, canvas_y);
                        }
                    }
                }
            }
        }
        MouseEventKind::Moved => {
            // Only track mouse cursor when Canvas panel is active
            if app.active_panel != Panel::Canvas {
                return;
            }

            // Update cursor position
            if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                app.update_cursor(canvas_x, canvas_y);
            }
        }
        MouseEventKind::Drag(_) => {
            if app.active_panel == Panel::Canvas {
                if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                    app.update_cursor(canvas_x, canvas_y);

                    if app.is_select_tool() {
                        // Selection mode: update selection or move
                        if app.is_in_selection_mode() {
                            if app.selection_state.mode == crate::app::SelectionMode::Selecting {
                                app.update_selection(canvas_x, canvas_y);
                            } else if app.selection_state.mode == crate::app::SelectionMode::Moving
                            {
                                app.update_move_selection(canvas_x, canvas_y);
                            }
                        }
                    } else {
                        // Handle dragging for drawing preview
                        if app.is_drawing() {
                            app.update_drawing(canvas_x, canvas_y);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Convert screen coordinates to canvas coordinates
fn to_canvas_coords(app: &App, column: u16, row: u16) -> Option<(u16, u16)> {
    if let Some(canvas_area) = app.canvas_area {
        let canvas_x = column.saturating_sub(canvas_area.x + 1);
        let canvas_y = row.saturating_sub(canvas_area.y + 1);

        // Check if within canvas bounds
        if canvas_x < canvas_area.width.saturating_sub(2)
            && canvas_y < canvas_area.height.saturating_sub(2)
        {
            return Some((canvas_x, canvas_y));
        }
    }
    None
}
