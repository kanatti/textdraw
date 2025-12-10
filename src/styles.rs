use ratatui::style::{Color, Style};

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
            label: Style::default().fg(Color::Yellow).bg(Color::DarkGray),
            value: Style::default().fg(Color::Cyan).bg(Color::DarkGray),
            background: Style::default().bg(Color::DarkGray),
        }
    } else if is_focused {
        // Focused: dark gray background
        InputStyles {
            label: Style::default().fg(Color::Yellow).bg(Color::DarkGray),
            value: Style::default().fg(Color::White).bg(Color::DarkGray),
            background: Style::default().bg(Color::DarkGray),
        }
    } else {
        // Normal: yellow label, default value
        InputStyles {
            label: Style::default().fg(Color::Yellow),
            value: Style::default(),
            background: Style::default(),
        }
    }
}
