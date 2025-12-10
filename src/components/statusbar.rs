use crate::components::Component;
use crate::events::EventHandler;
use crate::state::AppState;
use crate::tools::Tool;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
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

impl EventHandler for StatusBar {
    type State = AppState;
}

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

        // Split statusbar into left (tool & hints) and right (cursor)
        let chunks = Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]).split(area);

        // Left side: Mode/Tool indicator and hints (fixed width for consistent layout)
        let (mode_name, mode_bg_color) = if state.is_editing_table() {
            // Show EDIT TABLE mode
            (
                format!(" {:^width$} ", "EDIT TABLE", width = Tool::max_name_len()),
                Color::Magenta,
            )
        } else {
            // Show tool name
            let tool_name = format!(
                " {:^width$} ",
                state.tool.selected_tool.name(),
                width = Tool::max_name_len()
            );
            let tool_bg_color = if state.tool.selected_tool == Tool::Select {
                Color::Yellow
            } else {
                Color::Blue
            };
            (tool_name, tool_bg_color)
        };

        let mut left_spans = vec![
            Span::raw(" "),
            Span::styled(
                mode_name,
                Style::default().fg(Color::Black).bg(mode_bg_color),
            ),
        ];

        // Add contextual help based on mode and selection state
        if !state.is_editing_table() && state.is_select_tool() {
            let selected_ids = state.get_selected_element_ids();
            if !selected_ids.is_empty() {
                left_spans.push(Span::raw(" | Selected: "));
                left_spans.push(Span::styled(
                    selected_ids.len().to_string(),
                    Style::default().fg(Color::Yellow),
                ));

                // Show properties hint when exactly one element is selected
                if selected_ids.len() == 1 {
                    left_spans.push(Span::raw(" | Properties: "));
                    left_spans.push(Span::styled("p", Style::default().fg(Color::Cyan)));
                }

                // Show move/delete hints
                left_spans.push(Span::raw(" | Move: "));
                left_spans.push(Span::styled("←↑↓→", Style::default().fg(Color::Cyan)));
                left_spans.push(Span::raw(" | Delete: "));
                left_spans.push(Span::styled("⌫", Style::default().fg(Color::Cyan)));
            }
        }

        left_spans.push(Span::raw(" | Tools: "));
        left_spans.push(Span::styled("Space", Style::default().fg(Color::Cyan)));
        left_spans.push(Span::raw(" | Help: "));
        left_spans.push(Span::styled("?", Style::default().fg(Color::Cyan)));
        left_spans.push(Span::raw(" | Quit: "));
        left_spans.push(Span::styled("q", Style::default().fg(Color::Cyan)));

        let left_status =
            Paragraph::new(Line::from(left_spans)).style(Style::default().fg(Color::White));
        frame.render_widget(left_status, chunks[0]);

        // Right side: Cursor position (right-aligned)
        let right_spans = vec![
            Span::raw("Cursor: ("),
            Span::raw(state.cursor_x.to_string()),
            Span::raw(", "),
            Span::raw(state.cursor_y.to_string()),
            Span::raw(") "),
        ];
        let right_status = Paragraph::new(Line::from(right_spans))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Right);
        frame.render_widget(right_status, chunks[1]);
    }
}
