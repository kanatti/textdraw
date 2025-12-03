mod canvas;
mod sidebar;
mod statusbar;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

// Render the UI based on current App state.
pub fn render(frame: &mut Frame, app: &mut App) {
    let outer_layout = Layout::vertical([
        Constraint::Min(0),    // Main area
        Constraint::Length(1), // Status bar
    ])
    .split(frame.area());

    // Main area with sidebar and canvas
    let main_layout = Layout::horizontal([
        Constraint::Length(22), // Sidebar
        Constraint::Min(0),     // Canvas
    ])
    .split(outer_layout[0]);

    let sidebar_areas = sidebar::render(frame, main_layout[0], app);

    canvas::render(frame, main_layout[1], app);
    statusbar::render(frame, outer_layout[1], app);

    // Needed for event handling
    app.canvas_area = Some(main_layout[1]);
    app.tools_area = Some(sidebar_areas.0);
    app.elements_area = Some(sidebar_areas.1);
    app.properties_area = Some(sidebar_areas.2);
}
