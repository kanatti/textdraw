pub mod canvas;
pub mod help;
pub mod panels;
pub mod statusbar;

pub use canvas::CanvasComponent;
pub use help::HelpModal;
pub use panels::{ElementsPanel, PropertiesPanel, ToolsPanel};
pub use statusbar::StatusBar;

use crate::app::AppState;
use ratatui::Frame;

/// UI component that handles rendering with direct AppState access.
pub trait Component {
    /// Draw the component. Component reads its area from state.
    fn draw(&self, state: &AppState, frame: &mut Frame);
}
