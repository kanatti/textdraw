use crate::components::Component;
use crate::events::EventHandler;
use crate::state::AppState;
use crate::tools::Tool;
use crate::ui::{self, COLOR_ERROR, COLOR_HINT, COLOR_SUCCESS, CURSOR_BLOCK};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

const CURSOR_POSITION_WIDTH: u16 = 20;

// Mode badge background colors
const MODE_COLOR_SELECT: Color = Color::Yellow;
const MODE_COLOR_DRAW: Color = Color::Blue;
const MODE_COLOR_EDIT: Color = Color::Magenta;

// ============================================================================
// StatusBar Component
// ============================================================================

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

        if state.is_command_mode_active() {
            render_command_mode(state, frame, area);
            return;
        }

        if let Some(ref message) = state.file.status_message {
            render_status_message(message, frame, area);
            return;
        }

        // Default: Tool/mode info + hints + cursor position
        render_default_status(state, frame, area);
    }
}

/// Render command mode display
fn render_command_mode(state: &AppState, frame: &mut Frame, area: ratatui::layout::Rect) {
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

    spans.push(Span::styled(
        CURSOR_BLOCK,
        Style::default().fg(Color::White),
    ));

    let status = Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White));
    frame.render_widget(status, area);
}

/// Render status message (error or success)
fn render_status_message(message: &str, frame: &mut Frame, area: ratatui::layout::Rect) {
    let color = if message.starts_with("Error") {
        COLOR_ERROR
    } else {
        COLOR_SUCCESS
    };

    let text = format!(" {}", message);
    let status = Paragraph::new(text).style(Style::default().fg(color));
    frame.render_widget(status, area);
}

/// Render default status bar (mode badge + hints + cursor position)
fn render_default_status(state: &AppState, frame: &mut Frame, area: ratatui::layout::Rect) {
    let chunks = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(CURSOR_POSITION_WIDTH),
    ])
    .split(area);

    // Left side: mode badge + hints
    let left_status = build_left_status(state);
    frame.render_widget(left_status, chunks[0]);

    // Right side: cursor position
    let right_status = build_cursor_position(state);
    frame.render_widget(right_status, chunks[1]);
}

/// Build left side of status bar (mode badge + hints)
fn build_left_status(state: &AppState) -> Paragraph<'static> {
    let mut spans = vec![Span::raw(" ")];

    // Add mode badge
    let (badge_text, badge_color) = get_mode_badge(state);
    spans.push(create_mode_badge(&badge_text, badge_color));

    // Add contextual hints for select tool
    if !state.is_editing_table() && state.is_select_tool() {
        add_selection_hints(&mut spans, state);
    }

    // Add global hints
    add_global_hints(&mut spans);

    Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White))
}

/// Build right side of status bar (cursor position)
fn build_cursor_position(state: &AppState) -> Paragraph<'static> {
    let text = format!("Cursor: ({}, {}) ", state.cursor_x, state.cursor_y);
    Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Right)
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get mode badge text and color
fn get_mode_badge(state: &AppState) -> (String, Color) {
    if state.is_editing_table() {
        (
            format!(" {:^width$} ", "EDIT TABLE", width = Tool::max_name_len()),
            MODE_COLOR_EDIT,
        )
    } else {
        let tool_name = format!(
            " {:^width$} ",
            state.tool.selected_tool.name(),
            width = Tool::max_name_len()
        );
        let color = if state.tool.selected_tool == Tool::Select {
            MODE_COLOR_SELECT
        } else {
            MODE_COLOR_DRAW
        };
        (tool_name, color)
    }
}

/// Create a mode badge span
fn create_mode_badge(text: &str, bg_color: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(Color::Black).bg(bg_color),
    )
}

/// Add selection-specific hints to the status bar
fn add_selection_hints(spans: &mut Vec<Span<'static>>, state: &AppState) {
    let selected_ids = state.get_selected_element_ids();
    if selected_ids.is_empty() {
        return;
    }

    spans.push(ui::separator());
    spans.push(Span::raw("Selected: "));
    spans.push(Span::styled(
        selected_ids.len().to_string(),
        Style::default().fg(Color::Yellow),
    ));

    // Show properties hint when exactly one element is selected
    if selected_ids.len() == 1 {
        add_hint(spans, "Properties", "p");
    }

    // Show move/delete hints
    add_hint(spans, "Move", "←↑↓→");
    add_hint(spans, "Delete", "⌫");
}

/// Add global hints (tools, help, quit)
fn add_global_hints(spans: &mut Vec<Span<'static>>) {
    add_hint(spans, "Tools", "Space");
    add_hint(spans, "Help", "?");
    add_hint(spans, "Quit", "q");
}

/// Add a hint (label + key) to the spans
fn add_hint(spans: &mut Vec<Span<'static>>, label: &str, key: &str) {
    spans.push(ui::separator());
    spans.push(Span::raw(format!("{}: ", label)));
    spans.push(Span::styled(
        key.to_string(),
        Style::default().fg(COLOR_HINT),
    ));
}
