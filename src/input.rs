//! Input handling utilities for hit testing and coordinate detection.

use crate::types::{AppLayout, Coord, Panel};
use ratatui::layout::Rect;

/// Check if a coordinate is inside a rectangle
fn is_inside(coord: Coord, rect: Rect) -> bool {
    coord.x >= rect.x
        && coord.x < rect.x + rect.width
        && coord.y >= rect.y
        && coord.y < rect.y + rect.height
}

/// Static mapping of panel types to their layout accessor functions
static PANELS: &[(Panel, fn(&AppLayout) -> Option<Rect>)] = &[
    (Panel::Canvas, |l| l.canvas),
    (Panel::Tools, |l| l.tools),
    (Panel::Elements, |l| l.elements),
    (Panel::Properties, |l| l.properties),
];

/// Detect which panel was clicked based on mouse coordinates
pub fn detect_panel_click(coord: Coord, layout: &AppLayout) -> Option<Panel> {
    PANELS.iter().find_map(|(panel, get_area)| {
        get_area(layout)
            .filter(|rect| is_inside(coord, *rect))
            .map(|_| *panel)
    })
}
