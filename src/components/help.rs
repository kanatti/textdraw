use crate::components::Component;
use crate::components::help_line::HELP_LINES;
use crate::events::{EventHandler, EventResult};
use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};

pub struct HelpModal {
    scroll: u16,
}

impl HelpModal {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    fn scroll_down(&mut self, viewport_height: u16) {
        let max_scroll = self.max_scroll_for_viewport(viewport_height);
        self.scroll = self.scroll.saturating_add(1).min(max_scroll);
    }

    fn max_scroll_for_viewport(&self, viewport_height: u16) -> u16 {
        let total_lines = HELP_LINES.len() as u16;
        let visible_lines = viewport_height.saturating_sub(2); // Account for borders and padding
        total_lines.saturating_sub(visible_lines)
    }
}

impl EventHandler for HelpModal {
    type State = AppState;
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        if !state.show_help {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_up();
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // We need to calculate viewport height for scroll bounds
                // Use the modal height (60% of terminal)
                let terminal_height = state.layout.canvas.height;
                let viewport_height = (terminal_height * 60) / 100;
                self.scroll_down(viewport_height);
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_scroll(
        &mut self,
        state: &mut AppState,
        mouse_event: &MouseEvent,
    ) -> EventResult {
        if !state.show_help {
            return EventResult::Ignored;
        }

        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                self.scroll_up();
                EventResult::Consumed
            }
            MouseEventKind::ScrollDown => {
                let terminal_height = state.layout.canvas.height;
                let viewport_height = (terminal_height * 60) / 100;
                self.scroll_down(viewport_height);
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Component for HelpModal {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        if !state.show_help {
            return;
        }

        let area = Self::centered_rect(60, 60, frame.area());

        // Clear the area
        frame.render_widget(Clear, area);

        let help_text: Vec<Line> = HELP_LINES.iter().map(|line| line.to_line()).collect();

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green))
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .alignment(Alignment::Left)
            .scroll((self.scroll, 0));

        frame.render_widget(help, area);

        // Render scrollbar - inside the block area (not overlapping borders)
        let scrollbar_area = Rect {
            x: area.x + area.width - 1,
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let max_scroll = self.max_scroll_for_viewport(area.height);
        let mut scrollbar_state =
            ScrollbarState::new(max_scroll as usize).position(self.scroll as usize);

        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}
