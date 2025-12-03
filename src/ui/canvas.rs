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

    for y in 0..area.height.saturating_sub(2) {
        let mut line_chars = vec![];
        for x in 0..area.width.saturating_sub(2) {
            // Check if there's drawn content at this position
            let content = app.canvas.get(x as i32, y as i32);

            // Check if this position is part of the preview line being drawn
            let (is_preview, preview_char) = if let Some(drawing) = app.drawing {
                let dx = (app.cursor_x as i32 - drawing.start_x as i32).abs();
                let dy = (app.cursor_y as i32 - drawing.start_y as i32).abs();

                if dx > dy {
                    // Horizontal line
                    let (start_x, end_x) = if drawing.start_x <= app.cursor_x {
                        (drawing.start_x, app.cursor_x)
                    } else {
                        (app.cursor_x, drawing.start_x)
                    };
                    (y == drawing.start_y && x >= start_x && x <= end_x, '─')
                } else {
                    // Vertical line
                    let (start_y, end_y) = if drawing.start_y <= app.cursor_y {
                        (drawing.start_y, app.cursor_y)
                    } else {
                        (app.cursor_y, drawing.start_y)
                    };
                    (x == drawing.start_x && y >= start_y && y <= end_y, '│')
                }
            } else {
                (false, ' ')
            };

            if x == app.cursor_x && y == app.cursor_y {
                // Show cursor
                line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
            } else if is_preview {
                // Show preview line while drawing
                line_chars.push(Span::styled(
                    preview_char.to_string(),
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
