use crate::app::App;
use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

/// Result of handling an event - consumed or ignored for event propagation control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Event was handled, stop propagation
    Consumed,
    /// Event was not handled, continue to next component
    Ignored,
}

/// UI component that handles both rendering and events with direct App access.
pub trait Component {
    /// Handle event with mutable access to app state. Return Consumed to stop propagation.
    fn handle_event(&self, app: &mut App, event: &Event) -> EventResult {
        let _ = (app, event);
        EventResult::Ignored
    }

    /// Draw the component. This method is called every frame during render phase.
    fn draw(&self, app: &App, frame: &mut Frame, area: Rect);
}
