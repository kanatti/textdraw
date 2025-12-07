use crate::events::{ActionType, EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::input;
use crate::state::AppState;
use crate::tools::Tool;
use crate::types::{Coord, Panel};
use crossterm::event::{KeyCode, KeyModifiers};

/// Global event handler that handles fallthrough events not consumed by components
pub struct GlobalHandler;

impl EventHandler for GlobalHandler {
    type State = AppState;

    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Handle command mode if active
        if state.is_command_mode_active() {
            return match key_event.code {
                KeyCode::Char(c) => {
                    state.add_char_to_command(c);
                    EventResult::Consumed
                }
                KeyCode::Backspace => {
                    state.backspace_command();
                    EventResult::Consumed
                }
                KeyCode::Enter => {
                    state.execute_command();
                    EventResult::Consumed
                }
                KeyCode::Esc => {
                    state.exit_command_mode();
                    EventResult::Consumed
                }
                _ => EventResult::Consumed, // Consume all other keys while in command mode
            };
        }

        // Handle Ctrl+S and Ctrl+O - enter command mode with pre-filled command
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return match key_event.code {
                KeyCode::Char('s') => {
                    // Pre-fill with :save or :w
                    state.enter_command_mode_with("save ");
                    EventResult::Consumed
                }
                KeyCode::Char('o') => {
                    // Pre-fill with :open or :e
                    state.enter_command_mode_with("open ");
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            };
        }

        match key_event.code {
            KeyCode::Char(':') => {
                // Enter command mode
                state.enter_command_mode();
                EventResult::Consumed
            }
            KeyCode::Char('q') => EventResult::Action(ActionType::Quit),
            KeyCode::Char('?') => {
                state.toggle_help();
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
                state.switch_panel(panel);
                EventResult::Consumed
            }
            // Tool shortcuts
            KeyCode::Esc => {
                // Close help modal if open, otherwise switch to Select tool
                if state.show_help {
                    state.toggle_help();
                } else {
                    state.select_tool(Tool::Select);
                }
                EventResult::Consumed
            }
            // Tool selection - automatically handles all tools defined in types.rs
            KeyCode::Char(c) => {
                if let Some(tool) = Tool::from_key(c) {
                    state.select_tool(tool);
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Handle panel click - but if it's canvas, let it continue to canvas handling
        let coord = Coord {
            x: mouse_event.column,
            y: mouse_event.row,
        };
        let panel_click = input::detect_panel_click(coord, &state.layout);
        if let Some(panel) = panel_click {
            state.switch_panel(panel);
            // Only consume if it's NOT the canvas (canvas needs further processing by CanvasComponent)
            if panel != Panel::Canvas {
                return EventResult::Consumed;
            }
        }

        // Let canvas or other components handle it
        EventResult::Ignored
    }
}
