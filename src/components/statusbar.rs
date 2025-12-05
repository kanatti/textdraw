use crate::app::App;
use crate::components::Component;
use crate::types::EventHandler;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for StatusBar {}

impl Component for StatusBar {
    fn draw(&self, app: &App, frame: &mut Frame) {
        let Some(area) = app.layout.statusbar else {
            return;
        };

        let mut spans = vec![
            Span::raw(" Cursor: ("),
            Span::raw(app.cursor_x.to_string()),
            Span::raw(", "),
            Span::raw(app.cursor_y.to_string()),
            Span::raw(") | Tool: "),
            Span::styled(app.selected_tool.name(), Style::default().fg(Color::Yellow)),
        ];

        // Add contextual help based on selection state
        if app.is_select_tool() {
            let selected_ids = app.get_selected_element_ids();
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

        let status = Paragraph::new(Line::from(spans))
            .style(Style::default().fg(Color::White));

        frame.render_widget(status, area);
    }
}
