use crate::app::{App, Panel, Tool};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, MouseEventKind};

pub fn handle_event(app: &mut App, event: Event) -> Result<bool> {
    match event {
        Event::Key(key) => handle_key_event(app, key.code),
        Event::Mouse(mouse) => {
            handle_mouse_event(app, mouse.kind, mouse.column, mouse.row);
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn handle_key_event(app: &mut App, key_code: KeyCode) -> Result<bool> {
    // If text tool is active and in drawing mode, handle text input
    if app.is_text_input_mode() {
        return handle_text_input(app, key_code);
    }

    match key_code {
        KeyCode::Char('q') => Ok(true), // Signal to quit
        KeyCode::Char('0') => {
            app.switch_panel(Panel::Canvas);
            Ok(false)
        }
        KeyCode::Char('1') => {
            app.switch_panel(Panel::Tools);
            Ok(false)
        }
        KeyCode::Char('2') => {
            app.switch_panel(Panel::Elements);
            Ok(false)
        }
        KeyCode::Char('3') => {
            app.switch_panel(Panel::Properties);
            Ok(false)
        }
        // Tool shortcuts
        KeyCode::Char('s') | KeyCode::Esc => {
            app.select_tool(Tool::Select);
            Ok(false)
        }
        KeyCode::Char('l') => {
            app.select_tool(Tool::Line);
            Ok(false)
        }
        KeyCode::Char('r') => {
            app.select_tool(Tool::Rectangle);
            Ok(false)
        }
        KeyCode::Char('a') => {
            app.select_tool(Tool::Arrow);
            Ok(false)
        }
        KeyCode::Char('t') => {
            app.select_tool(Tool::Text);
            Ok(false)
        }
        // Arrow key navigation in Tools panel
        KeyCode::Up | KeyCode::Char('k') => {
            if app.active_panel == Panel::Tools {
                app.select_prev_tool();
            }
            Ok(false)
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.active_panel == Panel::Tools {
                app.select_next_tool();
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn handle_mouse_event(app: &mut App, kind: MouseEventKind, column: u16, row: u16) {
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

                // If clicking on canvas, start drawing with current tool
                if panel == Panel::Canvas {
                    if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                        app.start_drawing(canvas_x, canvas_y);
                    }
                }
            }
        }
        MouseEventKind::Up(_) => {
            // Finish drawing on mouse up (except for text tool which finishes on Enter)
            if app.is_drawing() && app.active_panel == Panel::Canvas && !app.is_text_input_mode() {
                if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                    app.finish_drawing(canvas_x, canvas_y);
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
            // Handle dragging for drawing preview
            if app.is_drawing() && app.active_panel == Panel::Canvas {
                if let Some((canvas_x, canvas_y)) = to_canvas_coords(app, column, row) {
                    app.update_cursor(canvas_x, canvas_y);
                    app.update_drawing(canvas_x, canvas_y);
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

/// Handle text input when text tool is active
fn handle_text_input(app: &mut App, key_code: KeyCode) -> Result<bool> {
    match key_code {
        KeyCode::Char(c) => {
            app.add_text_char(c);
            Ok(false)
        }
        KeyCode::Backspace => {
            app.text_backspace();
            Ok(false)
        }
        KeyCode::Enter | KeyCode::Esc => {
            // Commit or cancel text
            if key_code == KeyCode::Enter {
                app.finish_text_input();
            } else {
                app.cancel_drawing();
            }
            Ok(false)
        }
        _ => Ok(false),
    }
}
