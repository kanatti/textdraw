use crate::components::Component;
use crate::events::{EventHandler, EventResult};
use crate::state::AppState;
use crate::tools::Tool;
use crate::types::{Coord, Panel};
use crate::ui::UILayout;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct ToolsPanel;

impl ToolsPanel {
    pub fn new() -> Self {
        Self
    }

    /// Check if a coordinate is within the tools panel bounds
    fn is_within_panel(&self, coord: Coord, area: ratatui::layout::Rect) -> bool {
        coord.x >= area.x
            && coord.x < area.x + area.width
            && coord.y >= area.y
            && coord.y < area.y + area.height
    }

    /// Detect which tool was clicked based on mouse coordinates within the tools panel
    fn detect_tool_click(&self, coord: Coord, layout: &UILayout) -> Option<Tool> {
        let area = layout.tools;

        // Check if click is within the tools panel bounds
        if !self.is_within_panel(coord, area) {
            return None;
        }

        // Calculate relative Y position within tools panel
        let relative_y = coord.y.saturating_sub(area.y + 1); // +1 for border

        // Tools start at line 1 (after empty line), one tool per line
        let tool_index = relative_y.saturating_sub(1);
        let tools = Tool::all();

        if (tool_index as usize) < tools.len() {
            Some(tools[tool_index as usize])
        } else {
            None
        }
    }

    /// Detect if lock line was clicked (anywhere on the line within the tools panel)
    fn detect_lock_click(&self, coord: Coord, layout: &UILayout) -> bool {
        let area = layout.tools;

        // Check if click is within the tools panel bounds
        if !self.is_within_panel(coord, area) {
            return false;
        }

        // Calculate relative position within tools panel
        let relative_y = coord.y.saturating_sub(area.y + 1); // +1 for border

        // Lock checkbox is at line (tools.len() + 2)
        let lock_line = Tool::all().len() as u16 + 2;

        // Allow clicking anywhere on the lock line
        relative_y == lock_line
    }
}

impl EventHandler for ToolsPanel {
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Only handle when Tools panel is active
        if state.active_panel != Panel::Tools {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                state.select_prev_tool();
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.select_next_tool();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Only handle tool clicks when Tools panel is active
        if state.active_panel != Panel::Tools {
            return EventResult::Ignored;
        }

        // Check for lock checkbox click
        let coord = Coord {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        if self.detect_lock_click(coord, &state.layout) {
            state.toggle_tool_lock();
            return EventResult::Consumed;
        }

        // Check for tool click within the tools panel
        if let Some(tool) = self.detect_tool_click(coord, &state.layout) {
            state.select_tool(tool);
            return EventResult::Consumed;
        }

        EventResult::Ignored
    }
}

impl Component for ToolsPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        let area = state.layout.tools;

        let mut lines = vec![Line::from("")];

        for tool in Tool::all() {
            let is_selected = state.tool.selected_tool == tool;
            let key = tool.key().to_string();
            let name = tool.name().to_string();

            let line = if is_selected {
                Line::from(vec![
                    Span::styled(" [", Style::default().fg(Color::Yellow)),
                    Span::styled(key, Style::default().fg(Color::Yellow)),
                    Span::styled("] ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        name,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(" [", Style::default().fg(Color::DarkGray)),
                    Span::styled(key, Style::default().fg(Color::DarkGray)),
                    Span::styled("] ", Style::default().fg(Color::DarkGray)),
                    Span::raw(name),
                ])
            };

            lines.push(line);
        }

        // Add empty line separator
        lines.push(Line::from(""));

        // Add lock indicator
        let (icon, text, color) = if state.tool.tool_locked {
            ("✓", " Locked  ", Color::Green)
        } else {
            ("✗", " Unlocked", Color::Red)
        };
        let lock_line = Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(icon, Style::default().fg(color)),
            Span::styled(text, Style::default()),
        ]);
        lines.push(lock_line);

        let block = super::create_panel_block("[1]-Tools", Panel::Tools, state);
        let widget = Paragraph::new(lines).block(block);

        frame.render_widget(widget, area);
    }
}
