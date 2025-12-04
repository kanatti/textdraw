pub mod canvas;
pub mod panels;
pub mod statusbar;

pub use panels::{ElementsPanel, PropertiesPanel, ToolsPanel};
pub use statusbar::StatusBar;

use crate::app::App;
use crate::types::EventResult;
use crossterm::event::Event;
use ratatui::Frame;

/// UI component that handles both rendering and events with direct App access.
pub trait Component {
    /// Handle event with mutable access to app state. Return Consumed to stop propagation.
    fn handle_event(&self, app: &mut App, event: &Event) -> EventResult {
        let _ = (app, event);
        EventResult::Ignored
    }

    /// Draw the component. Component reads its area from app state.
    fn draw(&self, app: &App, frame: &mut Frame);
}
