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
        KeyCode::Char('b') => {
            app.select_tool(Tool::Box);
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
            // Check for tool click first
            if let Some(tool) = app.detect_tool_click(column, row) {
                app.select_tool(tool);
                app.switch_panel(Panel::Tools);
                return;
            }

            // Handle panel click
            if let Some(panel) = app.detect_panel_click(column, row) {
                app.switch_panel(panel);

                // If clicking on canvas with Line tool, start drawing
                if panel == Panel::Canvas && app.selected_tool == Tool::Line {
                    if let Some(canvas_area) = app.canvas_area {
                        let canvas_x = column.saturating_sub(canvas_area.x + 1);
                        let canvas_y = row.saturating_sub(canvas_area.y + 1);

                        if canvas_x < canvas_area.width.saturating_sub(2)
                            && canvas_y < canvas_area.height.saturating_sub(2)
                        {
                            app.start_drawing(canvas_x, canvas_y);
                        }
                    }
                }
            }
        }
        MouseEventKind::Up(_) => {
            // Finish drawing on mouse up
            if app.drawing.is_some() && app.active_panel == Panel::Canvas {
                if let Some(canvas_area) = app.canvas_area {
                    let canvas_x = column.saturating_sub(canvas_area.x + 1);
                    let canvas_y = row.saturating_sub(canvas_area.y + 1);

                    if canvas_x < canvas_area.width.saturating_sub(2)
                        && canvas_y < canvas_area.height.saturating_sub(2)
                    {
                        app.finish_drawing(canvas_x, canvas_y);
                    }
                }
            }
        }
        MouseEventKind::Moved => {
            // Only track mouse cursor when Canvas panel is active
            if app.active_panel != Panel::Canvas {
                return;
            }

            // Convert screen coordinates to canvas coordinates
            if let Some(canvas_area) = app.canvas_area {
                let canvas_x = column.saturating_sub(canvas_area.x + 1);
                let canvas_y = row.saturating_sub(canvas_area.y + 1);

                // Check if mouse is within canvas bounds
                if canvas_x < canvas_area.width.saturating_sub(2)
                    && canvas_y < canvas_area.height.saturating_sub(2)
                {
                    app.update_cursor(canvas_x, canvas_y);
                }
            }
        }
        MouseEventKind::Drag(_) => {
            // Handle dragging for drawing preview
            if app.drawing.is_some() && app.active_panel == Panel::Canvas {
                if let Some(canvas_area) = app.canvas_area {
                    let canvas_x = column.saturating_sub(canvas_area.x + 1);
                    let canvas_y = row.saturating_sub(canvas_area.y + 1);

                    // Check if mouse is within canvas bounds
                    if canvas_x < canvas_area.width.saturating_sub(2)
                        && canvas_y < canvas_area.height.saturating_sub(2)
                    {
                        app.update_cursor(canvas_x, canvas_y);
                    }
                }
            }
        }
        _ => {}
    }
}
