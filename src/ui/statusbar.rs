use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let status = Paragraph::new(Line::from(vec![
        Span::raw(" Cursor: ("),
        Span::raw(app.cursor_x.to_string()),
        Span::raw(", "),
        Span::raw(app.cursor_y.to_string()),
        Span::raw(") | Tool: "),
        Span::styled(app.selected_tool.name(), Style::default().fg(Color::Yellow)),
        Span::raw(" | 0:Canvas 1:Tools 2:Elements 3:Props | q:Quit"),
    ]))
    .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}
