pub mod canvas;
pub mod help;
mod help_line;
pub mod inputs;
pub mod panels;
pub mod statusbar;

pub use canvas::CanvasComponent;
pub use help::HelpModal;
pub use inputs::{ChoiceInput, NumericInput};
pub use panels::{PropertiesPanel, ToolsPanel};
pub use statusbar::StatusBar;

// Re-export the PropertyInput trait for use in other modules
pub use inputs::PropertyInput;

use crate::events::EventHandler;
use crate::state::AppState;
use ratatui::Frame;

/// UI component that handles rendering with direct AppState access.
/// All components must also implement EventHandler for event handling.
pub trait Component: EventHandler {
    /// Draw the component. Component reads its area from state.
    /// Uses &mut self to allow components to maintain local state.
    fn draw(&mut self, state: &AppState, frame: &mut Frame);
}
