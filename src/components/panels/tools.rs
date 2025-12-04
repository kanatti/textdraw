use crate::app::App;
use crate::components::Component;
use crate::types::{EventHandler, EventResult, Panel, Tool};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ToolsPanel;

impl ToolsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for ToolsPanel {
    fn handle_key_event(&self, app: &mut App, key_event: &KeyEvent) -> EventResult {
        // Only handle when Tools panel is active
        if app.active_panel != Panel::Tools {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_prev_tool();
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next_tool();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Component for ToolsPanel {
    fn draw(&self, app: &App, frame: &mut Frame) {
        let Some(area) = app.layout.tools else {
            return;
        };

        let mut lines = vec![Line::from("")];

        for tool in Tool::all() {
            let is_selected = app.selected_tool == tool;
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

        let block = super::create_panel_block("[1]-Tools", Panel::Tools, app.active_panel);
        let widget = Paragraph::new(lines).block(block);

        frame.render_widget(widget, area);
    }
}
