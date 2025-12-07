use crate::components::Component;
use crate::events::EventHandler;
use crate::state::AppState;
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for StatusBar {}

impl Component for StatusBar {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        let area = state.layout.statusbar;

        // If command mode is active, show it
        if state.is_command_mode_active() {
            // Split command from arguments (e.g., "save filename" -> "save" + " filename")
            let mut spans = vec![Span::raw(" ")];

            if let Some(space_idx) = state.command.buffer.find(' ') {
                // Has arguments - show command in yellow, args in white
                let (cmd, args) = state.command.buffer.split_at(space_idx);
                spans.push(Span::styled(
                    format!(":{}", cmd),
                    Style::default().fg(Color::Yellow),
                ));
                spans.push(Span::raw(args));
            } else {
                // No arguments - show entire thing in yellow
                spans.push(Span::styled(
                    format!(":{}", state.command.buffer),
                    Style::default().fg(Color::Yellow),
                ));
            }

            // Add cursor block
            spans.push(Span::styled("█", Style::default().fg(Color::White)));

            let status = Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White));

            frame.render_widget(status, area);
            return;
        }

        // If there's a status message, show it
        if let Some(ref message) = state.file.status_message {
            // Show errors in red, success messages in green
            let color = if message.starts_with("Error") {
                Color::Red
            } else {
                Color::Green
            };

            let spans = vec![
                Span::styled(" ", Style::default()),
                Span::styled(message, Style::default().fg(color)),
            ];

            let status = Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White));

            frame.render_widget(status, area);
            return;
        }

        // Normal statusbar
        let mut spans = vec![
            Span::raw(" Cursor: ("),
            Span::raw(state.cursor_x.to_string()),
            Span::raw(", "),
            Span::raw(state.cursor_y.to_string()),
            Span::raw(") | Tool: "),
            Span::styled(
                state.tool.selected_tool.name(),
                Style::default().fg(Color::Yellow),
            ),
        ];

        // Add contextual help based on selection state
        if state.is_select_tool() {
            let selected_ids = state.get_selected_element_ids();
            if !selected_ids.is_empty() {
                spans.push(Span::raw(" | Selected: "));
                spans.push(Span::styled(
                    selected_ids.len().to_string(),
                    Style::default().fg(Color::Yellow),
                ));
                spans.push(Span::raw(" | Move: "));
                spans.push(Span::styled("←↑↓→", Style::default().fg(Color::Cyan)));
                spans.push(Span::raw(" | Delete: "));
                spans.push(Span::styled("⌫", Style::default().fg(Color::Cyan)));
            }
        }

        spans.push(Span::raw(" | Help: "));
        spans.push(Span::styled("?", Style::default().fg(Color::Cyan)));
        spans.push(Span::raw(" | Quit: "));
        spans.push(Span::styled("q", Style::default().fg(Color::Cyan)));

        let status = Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White));

        frame.render_widget(status, area);
    }
}
