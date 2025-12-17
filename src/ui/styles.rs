use ratatui::style::{Color, Modifier, Style};

// ============================================================================
// Design System - Color Palette
// ============================================================================

/// Primary accent color used for borders, highlights, and active elements
pub const COLOR_PRIMARY: Color = Color::Cyan;

/// Color for labels and important text
pub const COLOR_LABEL: Color = Color::Yellow;

/// Color for selected/focused background
pub const COLOR_SELECTED_BG: Color = Color::DarkGray;

/// Color for muted/disabled text
pub const COLOR_MUTED: Color = Color::DarkGray;

/// Color for error messages
pub const COLOR_ERROR: Color = Color::Red;

/// Color for success messages
pub const COLOR_SUCCESS: Color = Color::Green;

/// Color for keybinding hints
pub const COLOR_HINT: Color = Color::Cyan;

// ============================================================================
// Common Spacing
// ============================================================================

/// Standard separator for statusbar hints
pub const SEPARATOR: &str = " | ";

/// Standard left padding for content (2 spaces)
pub const PADDING_LEFT: &str = "  ";

/// Width of right-side fill to extend input background across the line
pub const INPUT_BG_FILL_WIDTH: usize = 20;

// ============================================================================
// Common Symbols
// ============================================================================

/// Cursor block character used in canvas and statusbar
pub const CURSOR_BLOCK: &str = "█";

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
pub fn input_styles(is_editing: bool, is_focused: bool, panel_active: bool) -> InputStyles {
    // Only apply background highlight when panel is active
    if panel_active && is_editing {
        // Editing: dark gray background with cyan text
        InputStyles {
            label: Style::default().fg(COLOR_LABEL).bg(COLOR_SELECTED_BG),
            value: Style::default().fg(COLOR_PRIMARY).bg(COLOR_SELECTED_BG),
            background: Style::default().bg(COLOR_SELECTED_BG),
        }
    } else if panel_active && is_focused {
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
