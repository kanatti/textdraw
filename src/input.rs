//! Input handling utilities for hit testing and coordinate detection.

use crate::types::{Coord, Panel};
use crate::ui::UILayout;
use ratatui::layout::Rect;

/// Check if a coordinate is inside a rectangle
fn is_inside(coord: Coord, rect: Rect) -> bool {
    coord.x >= rect.x
        && coord.x < rect.x + rect.width
        && coord.y >= rect.y
        && coord.y < rect.y + rect.height
}

/// Static mapping of panel types to their layout accessor functions
/// Note: Properties panel is not included as it's now a floating overlay
static PANELS: &[(Panel, fn(&UILayout) -> Rect)] = &[
    (Panel::Canvas, |l| l.canvas),
    (Panel::Tools, |l| l.tools),
    (Panel::Elements, |l| l.elements),
];

/// Detect which panel was clicked based on mouse coordinates
pub fn detect_panel_click(coord: Coord, layout: &UILayout) -> Option<Panel> {
    PANELS.iter().find_map(|(panel, get_area)| {
        let rect = get_area(layout);
        if is_inside(coord, rect) {
            Some(*panel)
        } else {
            None
        }
    })
}
