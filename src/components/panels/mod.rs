mod elements;
mod properties;
mod tools;

pub use elements::ElementsPanel;
pub use properties::PropertiesPanel;
pub use tools::ToolsPanel;

use crate::types::Panel;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

/// Helper to create a styled panel block
pub(super) fn create_panel_block(title: &str, panel: Panel, active_panel: Panel) -> Block<'_> {
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
