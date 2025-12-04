mod canvas;
pub mod panels;
pub mod statusbar;

use crate::app::App;
use crate::component::Component;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

/// Render the UI based on current App state.
pub fn render(frame: &mut Frame, app: &mut App) {
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
        Constraint::Length(9), // Tools section
        Constraint::Length(9), // Elements section
        Constraint::Min(0),    // Properties section
    ])
    .split(main_layout[0]);

    // Draw panels using components
    app.tools_panel.draw(app, frame, panel_layout[0]);
    app.elements_panel.draw(app, frame, panel_layout[1]);
    app.properties_panel.draw(app, frame, panel_layout[2]);

    canvas::render(frame, main_layout[1], app);

    // Use component for statusbar
    app.statusbar.draw(app, frame, outer_layout[1]);

    // Store areas for event handling
    app.canvas_area = Some(main_layout[1]);
    app.tools_area = Some(panel_layout[0]);
    app.elements_area = Some(panel_layout[1]);
    app.properties_area = Some(panel_layout[2]);
}
