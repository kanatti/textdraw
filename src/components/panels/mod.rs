mod elements;
mod properties;
mod tools;

pub use elements::ElementsPanel;
pub use properties::PropertiesPanel;
pub use tools::ToolsPanel;

use crate::app::App;
use crate::types::Panel;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

/// Helper to create a styled panel block
pub(super) fn create_panel_block<'a>(title: &'a str, panel: Panel, app: &'a App) -> Block<'a> {
    let border_style = if !app.help.show && panel == app.active_panel {
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
