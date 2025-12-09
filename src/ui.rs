mod positioning;

pub use positioning::calculate_smart_position;

use crate::components::{
    CanvasComponent, Component, HelpModal, PropertiesPanel, StatusBar, ToolsPanel,
};
use crate::events::EventHandler;
use crate::state::AppState;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

/// Container for all UI components.
/// Components are created once and reused across renders, allowing them to maintain local state.
pub struct UI {
    tools_panel: ToolsPanel,
    properties_panel: PropertiesPanel,
    canvas: CanvasComponent,
    statusbar: StatusBar,
    help_modal: HelpModal,
}

impl UI {
    /// Create a new UI with all components initialized.
    pub fn new() -> Self {
        Self {
            tools_panel: ToolsPanel::new(),
            properties_panel: PropertiesPanel::new(),
            canvas: CanvasComponent::new(),
            statusbar: StatusBar::new(),
            help_modal: HelpModal::new(),
        }
    }

    /// Render all components.
    pub fn render(&mut self, frame: &mut Frame, state: &AppState) {
        self.canvas.draw(state, frame);
        self.statusbar.draw(state, frame);
        self.properties_panel.draw(state, frame); // Render after canvas as floating overlay
        self.tools_panel.draw(state, frame); // Render as floating modal
        self.help_modal.draw(state, frame);
    }

    /// Get event handlers in priority order for event dispatching.
    /// Returns mutable references to components for event handling.
    pub fn component_event_handlers(&mut self) -> Vec<&mut dyn EventHandler<State = AppState>> {
        vec![
            &mut self.help_modal,
            &mut self.tools_panel,
            &mut self.properties_panel,
            &mut self.canvas,
            &mut self.statusbar,
        ]
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UILayout {
    pub canvas: Rect,
    pub statusbar: Rect,
}

impl Default for UILayout {
    fn default() -> Self {
        Self {
            canvas: Rect::default(),
            statusbar: Rect::default(),
        }
    }
}

/// Calculate the layout of the UI.
pub fn calculate_layout(frame: &Frame) -> UILayout {
    let outer_layout = Layout::vertical([
        Constraint::Min(0),    // Main area (canvas takes full width)
        Constraint::Length(1), // Status bar
    ])
    .split(frame.area());

    UILayout {
        canvas: outer_layout[0],
        statusbar: outer_layout[1],
    }
}
