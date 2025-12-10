use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders},
};

// ============================================================================
// Design System - Color Palette
// ============================================================================

/// Primary accent color used for borders, highlights, and active elements
pub const COLOR_PRIMARY: Color = Color::Cyan;

/// Color for labels and important text
pub const COLOR_LABEL: Color = Color::Yellow;

/// Color for selected/focused background
pub const COLOR_SELECTED_BG: Color = Color::DarkGray;

/// Color for secondary accents (locked items, success states)
pub const COLOR_SUCCESS: Color = Color::Green;

/// Color for muted/disabled text
pub const COLOR_MUTED: Color = Color::DarkGray;

// ============================================================================
// Common Spacing
// ============================================================================

/// Standard left padding for content (2 spaces)
pub const PADDING_LEFT: &str = "  ";

/// Standard padding width for filling lines
pub const PADDING_FILL_WIDTH: usize = 20;

// ============================================================================
// Input Styles
// ============================================================================

/// Styles for property input widgets
pub struct InputStyles {
    pub label: Style,
    pub value: Style,
    pub background: Style,
}

/// Get styles for property inputs based on state
pub fn input_styles(is_editing: bool, is_focused: bool) -> InputStyles {
    if is_editing {
        // Editing: dark gray background with cyan text
        InputStyles {
            label: Style::default().fg(COLOR_LABEL).bg(COLOR_SELECTED_BG),
            value: Style::default().fg(COLOR_PRIMARY).bg(COLOR_SELECTED_BG),
            background: Style::default().bg(COLOR_SELECTED_BG),
        }
    } else if is_focused {
        // Focused: dark gray background
        InputStyles {
            label: Style::default().fg(COLOR_LABEL).bg(COLOR_SELECTED_BG),
            value: Style::default().fg(Color::White).bg(COLOR_SELECTED_BG),
            background: Style::default().bg(COLOR_SELECTED_BG),
        }
    } else {
        // Normal: yellow label, default value
        InputStyles {
            label: Style::default().fg(COLOR_LABEL),
            value: Style::default(),
            background: Style::default(),
        }
    }
}

// ============================================================================
// Common Text Styles
// ============================================================================

/// Style for labels (yellow)
pub fn label_style() -> Style {
    Style::default().fg(COLOR_LABEL)
}

/// Style for muted/helper text (dark gray)
pub fn muted_style() -> Style {
    Style::default().fg(COLOR_MUTED)
}

/// Style for section headers (bold)
pub fn header_style() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}

// ============================================================================
// Common Line Constructors
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

/// Create a padded span with background fill
pub fn padding_span(width: usize, style: Style) -> Span<'static> {
    Span::styled(" ".repeat(width), style)
}

/// Create a styled span with standard left padding prepended
pub fn padded_span(text: impl Into<String>, style: Style) -> Span<'static> {
    Span::styled(format!("{}{}", PADDING_LEFT, text.into()), style)
}

// ============================================================================
// Common Block Styles
// ============================================================================

/// Create a standard modal block with rounded borders and cyan accent
pub fn modal_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!(" {} ", title))
        .border_style(Style::default().fg(COLOR_PRIMARY))
}
