pub mod canvas;
pub mod help;
pub mod panels;
pub mod statusbar;

pub use canvas::CanvasComponent;
pub use help::HelpModal;
pub use panels::{ElementsPanel, PropertiesPanel, ToolsPanel};
pub use statusbar::StatusBar;

use crate::app::App;
use ratatui::Frame;

/// UI component that handles rendering with direct App access.
pub trait Component {
    /// Draw the component. Component reads its area from app state.
    fn draw(&self, app: &App, frame: &mut Frame);
}
