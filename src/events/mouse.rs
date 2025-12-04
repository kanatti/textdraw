use crate::app::App;
use crate::types::{Panel, SelectionMode};
use crossterm::event::MouseEventKind;

pub fn handle_mouse_event(app: &mut App, kind: MouseEventKind, column: u16, row: u16) {
    match kind {
        MouseEventKind::Down(_) => handle_mouse_down(app, column, row),
        MouseEventKind::Up(_) => handle_mouse_up(app, column, row),
        MouseEventKind::Moved => handle_mouse_moved(app, column, row),
        MouseEventKind::Drag(_) => handle_mouse_drag(app, column, row),
        _ => {}
    }
}

fn handle_mouse_down(app: &mut App, column: u16, row: u16) {
    // If text tool is active and we're in text input mode, finish the text
    if app.is_text_input_mode() {
        app.finish_text_input();
    }

    // Try to handle as UI click (tool/panel)
    if handle_ui_click(app, column, row) {
        return;
    }

    // Handle canvas click
    handle_canvas_mouse_down(app, column, row);
}

/// Handle UI clicks (tools, panels). Returns true if fully handled (should not process canvas).
fn handle_ui_click(app: &mut App, column: u16, row: u16) -> bool {
    // Check for tool click first
    if let Some(tool) = app.detect_tool_click(column, row) {
        app.select_tool(tool);
        app.switch_panel(Panel::Tools);
        return true;
    }

    // Handle panel click - but if it's canvas, let it continue to canvas handling
    if let Some(panel) = app.detect_panel_click(column, row) {
        app.switch_panel(panel);
        // Only return true if it's NOT the canvas (canvas needs further processing)
        return panel != Panel::Canvas;
    }

    false
}

/// Handle canvas clicks (selection/drawing)
fn handle_canvas_mouse_down(app: &mut App, column: u16, row: u16) {
    if app.active_panel != Panel::Canvas {
        return;
    }

    let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) else {
        return;
    };

    if app.is_select_tool() {
        handle_selection_mouse_down(app, canvas_x, canvas_y);
    } else {
        app.start_drawing(canvas_x, canvas_y);
    }
}

/// Handle mouse down in selection mode
fn handle_selection_mouse_down(app: &mut App, canvas_x: u16, canvas_y: u16) {
    if app.is_in_selection_mode() {
        // Check if clicking inside any selected element's bounds
        let clicked_selected = is_clicking_selected_element(app, canvas_x, canvas_y);

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
}

/// Check if click is inside any selected element
fn is_clicking_selected_element(app: &App, canvas_x: u16, canvas_y: u16) -> bool {
    let px = canvas_x as i32;
    let py = canvas_y as i32;

    for element_id in app.get_selected_element_ids() {
        if let Some(element) = app.canvas.get_element(*element_id) {
            if element.point_in_bounds(px, py) {
                return true;
            }
        }
    }

    false
}

fn handle_mouse_up(app: &mut App, column: u16, row: u16) {
    if app.active_panel == Panel::Canvas {
        if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
            if app.is_select_tool() {
                // Selection mode: finish selection or move
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

fn handle_mouse_moved(app: &mut App, column: u16, row: u16) {
    // Only track mouse cursor when Canvas panel is active
    if app.active_panel != Panel::Canvas {
        return;
    }

    // Update cursor position
    if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
        app.update_cursor(canvas_x, canvas_y);
    }
}

fn handle_mouse_drag(app: &mut App, column: u16, row: u16) {
    if app.active_panel == Panel::Canvas {
        if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
            app.update_cursor(canvas_x, canvas_y);

            if app.is_select_tool() {
                // Selection mode: update selection or move
                if app.is_in_selection_mode() {
                    if app.selection_state.mode == SelectionMode::Selecting {
                        app.update_selection(canvas_x, canvas_y);
                    } else if app.selection_state.mode == SelectionMode::Moving
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

/// Convert screen coordinates to canvas coordinates
fn to_canvas_coords(app: &App, column: u16, row: u16) -> Option<(u16, u16)> {
    if let Some(canvas_area) = app.layout.canvas {
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
