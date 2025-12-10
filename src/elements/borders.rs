use serde::{Deserialize, Serialize};

/// Character set for drawing borders with all junction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    // Junction characters for tables
    pub cross: char,     // Full intersection (┼)
    pub left_t: char,    // Left T-junction (├)
    pub right_t: char,   // Right T-junction (┤)
    pub top_t: char,     // Top T-junction (┬)
    pub bottom_t: char,  // Bottom T-junction (┴)
}

/// Border style with support for box drawing characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BorderStyle {
    #[default]
    Single,
    Double,
    Bold,
    Rounded,
    None,
}

impl BorderStyle {
    /// Get the box drawing characters for this border style
    pub fn chars(&self) -> BorderChars {
        match self {
            BorderStyle::Single => BorderChars {
                horizontal: '─',
                vertical: '│',
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                cross: '┼',
                left_t: '├',
                right_t: '┤',
                top_t: '┬',
                bottom_t: '┴',
            },
            BorderStyle::Double => BorderChars {
                horizontal: '═',
                vertical: '║',
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                cross: '╬',
                left_t: '╠',
                right_t: '╣',
                top_t: '╦',
                bottom_t: '╩',
            },
            BorderStyle::Bold => BorderChars {
                horizontal: '━',
                vertical: '┃',
                top_left: '┏',
                top_right: '┓',
                bottom_left: '┗',
                bottom_right: '┛',
                cross: '╋',
                left_t: '┣',
                right_t: '┫',
                top_t: '┳',
                bottom_t: '┻',
            },
            BorderStyle::Rounded => BorderChars {
                horizontal: '─',
                vertical: '│',
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                // Rounded doesn't have special junction chars, use single
                cross: '┼',
                left_t: '├',
                right_t: '┤',
                top_t: '┬',
                bottom_t: '┴',
            },
            BorderStyle::None => BorderChars {
                horizontal: ' ',
                vertical: ' ',
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                cross: ' ',
                left_t: ' ',
                right_t: ' ',
                top_t: ' ',
                bottom_t: ' ',
            },
        }
    }

    /// Convert border style to string for property value
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderStyle::Single => "Single",
            BorderStyle::Double => "Double",
            BorderStyle::Bold => "Bold",
            BorderStyle::Rounded => "Rounded",
            BorderStyle::None => "None",
        }
    }

    /// Parse border style from string
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Single" => Ok(BorderStyle::Single),
            "Double" => Ok(BorderStyle::Double),
            "Bold" => Ok(BorderStyle::Bold),
            "Rounded" => Ok(BorderStyle::Rounded),
            "None" => Ok(BorderStyle::None),
            _ => anyhow::bail!("Invalid border style: {}", s),
        }
    }

    /// Get all available border styles as strings (for property choices)
    pub fn all_options() -> Vec<String> {
        vec![
            "Single".to_string(),
            "Double".to_string(),
            "Bold".to_string(),
            "Rounded".to_string(),
            "None".to_string(),
        ]
    }
}
