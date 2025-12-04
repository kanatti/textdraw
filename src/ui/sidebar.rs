use crate::app::App;
use crate::types::{Panel, Tool};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) -> (Rect, Rect, Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // Tools section
            Constraint::Length(9), // Elements section
            Constraint::Min(0),    // Properties section
        ])
        .split(area);

    render_tools(frame, sections[0], app);
    render_elements(frame, sections[1], app);
    render_properties(frame, sections[2], app);

    // Return the areas for click detection
    (sections[0], sections[1], sections[2])
}

/// Helper to create a styled sidebar panel block
fn create_panel_block(title: &str, panel: Panel, active_panel: Panel) -> Block<'_> {
    let border_style = if panel == active_panel {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(border_style)
}

fn render_tools(frame: &mut Frame, area: Rect, app: &App) {
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

    let block = create_panel_block("[1]-Tools", Panel::Tools, app.active_panel);
    let widget = Paragraph::new(lines).block(block);

    frame.render_widget(widget, area);
}

fn render_elements(frame: &mut Frame, area: Rect, app: &App) {
    let elements = vec![Line::from(""), Line::from("  (empty)")];

    let block = create_panel_block("[2]-Elements", Panel::Elements, app.active_panel);
    let widget = Paragraph::new(elements).block(block);

    frame.render_widget(widget, area);
}

fn render_properties(frame: &mut Frame, area: Rect, app: &App) {
    let props = vec![Line::from(""), Line::from("  (no selection)")];

    let block = create_panel_block("[3]-Properties", Panel::Properties, app.active_panel);
    let widget = Paragraph::new(props).block(block);

    frame.render_widget(widget, area);
}
