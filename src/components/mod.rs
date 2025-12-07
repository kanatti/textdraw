pub mod canvas;
pub mod help;
mod help_line;
pub mod panels;
pub mod statusbar;

pub use canvas::CanvasComponent;
pub use help::HelpModal;
pub use panels::{ElementsPanel, PropertiesPanel, ToolsPanel};
pub use statusbar::StatusBar;

use crate::state::AppState;
use ratatui::Frame;

/// UI component that handles rendering with direct AppState access.
pub trait Component {
    /// Draw the component. Component reads its area from state.
    /// Uses &mut self to allow components to maintain local state.
    fn draw(&mut self, state: &AppState, frame: &mut Frame);
}
