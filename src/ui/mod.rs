mod canvas;
mod sidebar;
mod statusbar;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct PanelAreas {
    pub canvas: Rect,
    pub tools: Rect,
    pub elements: Rect,
    pub properties: Rect,
}

pub fn render(frame: &mut Frame, app: &App) -> PanelAreas {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    // Main area with sidebar and canvas
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // Sidebar
            Constraint::Min(0),     // Canvas
        ])
        .split(layout[0]);

    let sidebar_areas = sidebar::render(frame, main_layout[0], app);
    canvas::render(frame, main_layout[1], app);
    statusbar::render(frame, layout[1], app);

    PanelAreas {
        canvas: main_layout[1],
        tools: sidebar_areas.0,
        elements: sidebar_areas.1,
        properties: sidebar_areas.2,
    }
}
