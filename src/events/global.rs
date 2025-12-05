use crate::app::App;
use crate::input;
use crate::types::{ActionType, Coord, EventHandler, EventResult, Panel, Tool};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};

/// Global event handler that handles fallthrough events not consumed by components
pub struct GlobalHandler;

impl EventHandler for GlobalHandler {
    fn handle_key_event(&self, app: &mut App, key_event: &KeyEvent) -> EventResult {
        match key_event.code {
            KeyCode::Char('q') => EventResult::Action(ActionType::Quit),
            KeyCode::Char('?') => {
                app.toggle_help();
                EventResult::Consumed
            }
            // Panel shortcuts
            KeyCode::Char(c @ '0'..='3') => {
                let panel = match c {
                    '0' => Panel::Canvas,
                    '1' => Panel::Tools,
                    '2' => Panel::Elements,
                    '3' => Panel::Properties,
                    _ => unreachable!("Unhandled panel switch"),
                };
                app.switch_panel(panel);
                EventResult::Consumed
            }
            // Tool shortcuts
            KeyCode::Esc => {
                // Close help modal if open, otherwise switch to Select tool
                if app.show_help {
                    app.toggle_help();
                } else {
                    app.select_tool(Tool::Select);
                }
                EventResult::Consumed
            }
            // Tool selection - automatically handles all tools defined in types.rs
            KeyCode::Char(c) => {
                if let Some(tool) = Tool::from_key(c) {
                    app.select_tool(tool);
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(&self, app: &mut App, mouse_event: &MouseEvent) -> EventResult {
        // Handle panel click - but if it's canvas, let it continue to canvas handling
        let coord = Coord {
            x: mouse_event.column,
            y: mouse_event.row,
        };
        let panel_click = input::detect_panel_click(coord, &app.layout);
        if let Some(panel) = panel_click {
            app.switch_panel(panel);
            // Only consume if it's NOT the canvas (canvas needs further processing by CanvasComponent)
            if panel != Panel::Canvas {
                return EventResult::Consumed;
            }
        }

        // Let canvas or other components handle it
        EventResult::Ignored
    }
}
