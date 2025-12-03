use crate::app::{App, Panel};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = vec![];

    // Get preview points from the active tool
    let preview_points = app.get_preview_points();
    let preview_map: std::collections::HashMap<(i32, i32), char> = preview_points
        .into_iter()
        .map(|(x, y, ch)| ((x, y), ch))
        .collect();

    for y in 0..area.height.saturating_sub(2) {
        let mut line_chars = vec![];
        for x in 0..area.width.saturating_sub(2) {
            // Check if there's drawn content at this position
            let content = app.canvas.get(x as i32, y as i32);

            // Check if this position is part of the preview
            let preview_char = preview_map.get(&(x as i32, y as i32));

            if x == app.cursor_x && y == app.cursor_y {
                // Show cursor
                line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
            } else if let Some(&ch) = preview_char {
                // Show preview while drawing
                line_chars.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            } else if let Some(ch) = content {
                // Show drawn content
                line_chars.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::White),
                ));
            } else {
                // Empty space
                line_chars.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(line_chars));
    }

    let canvas_style = if app.active_panel == Panel::Canvas {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let canvas = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("[0]-Canvas")
            .border_style(canvas_style),
    );

    frame.render_widget(canvas, area);
}
