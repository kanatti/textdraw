use crate::app::App;
use crate::types::{Panel, SelectionMode};
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

    // Get selection box (grey) for drag-select
    let selection_box = app.get_selection_box_points();
    let selection_box_map: std::collections::HashMap<(i32, i32), char> = selection_box
        .into_iter()
        .map(|(x, y, ch)| ((x, y), ch))
        .collect();

    // Get selected element IDs and move offset
    let selected_ids = app.get_selected_element_ids();
    let move_offset = app.get_move_offset();

    // Build render cache: map (x, y) -> (char, element_id)
    // This is O(total_points) instead of O(pixels × elements)
    let mut render_map: std::collections::HashMap<(i32, i32), (char, usize)> =
        std::collections::HashMap::new();

    for element in app.canvas.elements() {
        let element_id = element.id();
        let is_selected = selected_ids.contains(&element_id);

        // Calculate offset for selected elements being moved
        let (offset_x, offset_y) = if is_selected {
            move_offset.unwrap_or((0, 0))
        } else {
            (0, 0)
        };

        // Add all points from this element to render map (with offset if moving)
        for ((x, y), ch) in element.points() {
            let render_x = x + offset_x;
            let render_y = y + offset_y;
            render_map.insert((render_x, render_y), (*ch, element_id));
        }
    }

    for y in 0..area.height.saturating_sub(2) {
        let mut line_chars = vec![];
        for x in 0..area.width.saturating_sub(2) {
            let px = x as i32;
            let py = y as i32;

            // Priority: cursor > selection box > preview > elements

            // Check if actively selecting or moving
            let is_actively_selecting_or_moving = matches!(
                app.selection_state.mode,
                SelectionMode::Selecting | SelectionMode::Moving
            );

            // Check if hovering over a selected element (to hide cursor)
            let mut hovering_selected = false;
            if app.is_select_tool() && !selected_ids.is_empty() {
                for element_id in selected_ids {
                    if let Some(element) = app.canvas.get_element(*element_id) {
                        if element.point_in_bounds(px, py) {
                            hovering_selected = true;
                            break;
                        }
                    }
                }
            }

            if x == app.cursor_x
                && y == app.cursor_y
                && !app.is_drawing()
                && !is_actively_selecting_or_moving
                && !hovering_selected
            {
                // Show cursor block only when:
                // - Not drawing
                // - Not actively selecting/moving
                // - Not hovering over a selected element
                line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
            } else if let Some(&ch) = selection_box_map.get(&(px, py)) {
                // Show selection box in grey (during drag-select)
                line_chars.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            } else if let Some(&ch) = preview_map.get(&(px, py)) {
                // Show preview while drawing
                let preview_color = Color::DarkGray;
                line_chars.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(preview_color),
                ));
            } else if let Some((ch, element_id)) = render_map.get(&(px, py)) {
                // Found element at this position - O(1) lookup!
                let is_selected = selected_ids.contains(element_id);
                let color = if is_selected {
                    Color::Yellow // Selected elements in yellow
                } else {
                    Color::White // Normal elements in white
                };
                line_chars.push(Span::styled(ch.to_string(), Style::default().fg(color)));
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
