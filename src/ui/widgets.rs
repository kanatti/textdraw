use super::styles::{
    INPUT_BG_FILL_WIDTH, InputStyles, PADDING_LEFT, SEPARATOR, header_style, label_style,
};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

// ============================================================================
// Block Widgets
// ============================================================================

/// Create a panel block with active/inactive border styling
pub fn panel_block(title: &str, is_active: bool) -> Block<'_> {
    let border_color = if is_active {
        Color::Green
    } else {
        Color::DarkGray
    };

    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
}

// ============================================================================
// Line and Span Constructors
// ============================================================================

/// Create a blank line
pub fn blank_line() -> Line<'static> {
    Line::from("")
}

/// Create a label-value line with standard padding
pub fn label_value_line(label: &str, value: impl Into<String>) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}{}: ", PADDING_LEFT, label), label_style()),
        Span::raw(value.into()),
    ])
}

/// Create a section header line
pub fn section_header(text: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        format!("{}{}:", PADDING_LEFT, text),
        header_style(),
    )])
}

/// Create a styled span with standard left padding prepended
pub fn padded_span(text: impl Into<String>, style: Style) -> Span<'static> {
    Span::styled(format!("{}{}", PADDING_LEFT, text.into()), style)
}

/// Create a separator span (e.g., " | " for statusbar)
pub fn separator() -> Span<'static> {
    Span::raw(SEPARATOR)
}

// ============================================================================
// Input Widgets
// ============================================================================

/// Create an input line with label, value, and background padding
pub fn input_line(label: &str, value: String, styles: InputStyles) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}{}: ", PADDING_LEFT, label), styles.label),
        Span::styled(value, styles.value),
        Span::styled(" ".repeat(INPUT_BG_FILL_WIDTH), styles.background),
    ])
}
