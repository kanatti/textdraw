use crate::components::Component;
use crate::events::{EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::state::AppState;
use crate::tools::Tool;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub struct ToolsPanel;

impl ToolsPanel {
    pub fn new() -> Self {
        Self
    }

    /// Calculate modal position at bottom-left corner
    fn calculate_modal_area(canvas_area: Rect) -> Rect {
        const MODAL_WIDTH: u16 = 25;
        const MODAL_HEIGHT: u16 = 12;

        Rect {
            x: canvas_area.x + 2,
            y: canvas_area.y + canvas_area.height - MODAL_HEIGHT - 1,
            width: MODAL_WIDTH,
            height: MODAL_HEIGHT,
        }
    }

    /// Check if coordinate is within modal area
    fn is_inside_modal(x: u16, y: u16, modal_area: Rect) -> bool {
        x >= modal_area.x
            && x < modal_area.x + modal_area.width
            && y >= modal_area.y
            && y < modal_area.y + modal_area.height
    }

    /// Detect which tool was clicked based on Y coordinate within modal
    fn detect_tool_click(y: u16, modal_area: Rect) -> Option<Tool> {
        // Calculate relative Y position within modal (account for border + empty line at top)
        let relative_y = y.saturating_sub(modal_area.y + 2); // +1 border +1 empty line

        let tools = Tool::all();
        let tool_index = relative_y as usize;

        if tool_index < tools.len() {
            Some(tools[tool_index])
        } else {
            None
        }
    }

    /// Detect if lock line was clicked
    fn is_lock_click(y: u16, modal_area: Rect) -> bool {
        let relative_y = y.saturating_sub(modal_area.y + 1); // +1 for border

        // Lock line is at: empty line (1) + tools (5) + empty (1) + separator (1) + lock line
        // = 1 + 5 + 1 + 1 + 0 = 8 (0-indexed from line after border)
        let lock_line = 1 + Tool::all().len() as u16 + 1 + 1;

        relative_y == lock_line
    }
}

impl EventHandler for ToolsPanel {
    type State = AppState;
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Only handle when tools modal is visible
        if !state.show_tools_modal {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char(' ') => {
                state.toggle_tools_modal();
                EventResult::Consumed
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.select_prev_tool();
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.select_next_tool();
                EventResult::Consumed
            }
            KeyCode::Enter => {
                // Select tool (tool is already selected through navigation, no need to close)
                EventResult::Consumed
            }
            KeyCode::Tab | KeyCode::Char('x') => {
                state.toggle_tool_lock();
                EventResult::Consumed
            }
            KeyCode::Char(c) => {
                // Direct tool selection by shortcut (stays open)
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
        // Only handle when modal is visible
        if !state.show_tools_modal {
            return EventResult::Ignored;
        }

        let canvas_area = state.layout.canvas;
        let modal_area = Self::calculate_modal_area(canvas_area);

        // Check if click is within modal bounds
        if !Self::is_inside_modal(mouse_event.column, mouse_event.row, modal_area) {
            return EventResult::Ignored;
        }

        // Check for lock toggle click
        if Self::is_lock_click(mouse_event.row, modal_area) {
            state.toggle_tool_lock();
            return EventResult::Consumed;
        }

        // Check for tool click
        if let Some(tool) = Self::detect_tool_click(mouse_event.row, modal_area) {
            state.select_tool(tool);
            return EventResult::Consumed;
        }

        EventResult::Ignored
    }
}

impl Component for ToolsPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        // Only render if modal is visible
        if !state.show_tools_modal {
            return;
        }

        let canvas_area = state.layout.canvas;
        let modal_area = Self::calculate_modal_area(canvas_area);

        // Clear the area behind the modal
        frame.render_widget(Clear, modal_area);

        let mut lines = vec![Line::from("")];

        for tool in Tool::all() {
            let is_selected = state.tool.selected_tool == tool;
            let key = tool.key().to_string();
            let name = tool.name().to_string();

            let (prefix, style) = if is_selected {
                (
                    "→ ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ("  ", Style::default())
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{} ", key), Style::default().fg(Color::Cyan)),
                Span::styled(name, style),
            ]);

            lines.push(line);
        }

        // Add separator
        lines.push(Line::from(""));
        lines.push(Line::from("─────────────────────"));

        // Add lock toggle
        let lock_text = if state.tool.tool_locked { "[x]" } else { "[ ]" };
        let lock_style = if state.tool.tool_locked {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        lines.push(Line::from(vec![
            Span::styled(lock_text, lock_style),
            Span::raw(" Lock tool"),
        ]));

        let block = Block::default()
            .title("Tools")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan));

        let widget = Paragraph::new(lines).block(block);

        frame.render_widget(widget, modal_area);
    }
}
