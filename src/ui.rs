use crate::app::App;
use crate::components::{CanvasComponent, Component, ElementsPanel, HelpModal, PropertiesPanel, StatusBar, ToolsPanel};
use crate::types::AppLayout;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

/// Render the UI based on current App state.
pub fn render(frame: &mut Frame, app: &App) {
    // Draw components
    let components: Vec<Box<dyn Component>> = vec![
        Box::new(ToolsPanel::new()),
        Box::new(ElementsPanel::new()),
        Box::new(PropertiesPanel::new()),
        Box::new(CanvasComponent::new()),
        Box::new(StatusBar::new()),
        Box::new(HelpModal::new()),
    ];

    for component in components {
        component.draw(app, frame);
    }
}

/// Calculate the layout of the UI.
pub fn calculate_layout(frame: &Frame) -> AppLayout {
    let outer_layout = Layout::vertical([
        Constraint::Min(0),    // Main area
        Constraint::Length(1), // Status bar
    ])
    .split(frame.area());

    // Main area with side panels and canvas
    let main_layout = Layout::horizontal([
        Constraint::Length(22), // Side panels
        Constraint::Min(0),     // Canvas
    ])
    .split(outer_layout[0]);

    // Split side into panels
    let panel_layout = Layout::vertical([
        Constraint::Length(11), // Tools section (5 tools + empty + lock + borders)
        Constraint::Length(9),  // Elements section
        Constraint::Min(0),     // Properties section
    ])
    .split(main_layout[0]);

    AppLayout {
        canvas: Some(main_layout[1]),
        tools: Some(panel_layout[0]),
        elements: Some(panel_layout[1]),
        properties: Some(panel_layout[2]),
        statusbar: Some(outer_layout[1]),
    }
}