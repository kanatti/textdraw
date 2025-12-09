use ratatui::layout::Rect;

/// Gap between element and panel
const POSITIONING_GAP: u16 = 2;

/// Calculate smart position for a floating panel relative to an element.
///
/// Priority: Right of element > Left of element > Below > Above > Center (fallback)
///
/// **Arguments**:
///
/// * `elem_x` - Element's screen X coordinate (absolute)
/// * `elem_y` - Element's screen Y coordinate (absolute)
/// * `elem_width` - Element width
/// * `elem_height` - Element height
/// * `canvas_area` - Canvas bounds (screen coordinates)
/// * `panel_width` - Panel width
/// * `panel_height` - Panel height
///
/// **Returns**:
///
/// A `Rect` representing the panel position in screen coordinates
pub fn calculate_smart_position(
    elem_x: u16,
    elem_y: u16,
    elem_width: u16,
    elem_height: u16,
    canvas_area: Rect,
    panel_width: u16,
    panel_height: u16,
) -> Rect {
    let elem_right = elem_x + elem_width;

    // Try right side first (most common for properties)
    let right_x = elem_right + POSITIONING_GAP;
    if right_x + panel_width <= canvas_area.x + canvas_area.width {
        // Align vertically with element top, but keep within canvas
        let y = elem_y.min(canvas_area.y + canvas_area.height.saturating_sub(panel_height));
        return Rect {
            x: right_x,
            y,
            width: panel_width,
            height: panel_height,
        };
    }

    // Try left side
    if elem_x >= canvas_area.x + panel_width + POSITIONING_GAP {
        let left_x = elem_x - panel_width - POSITIONING_GAP;
        let y = elem_y.min(canvas_area.y + canvas_area.height.saturating_sub(panel_height));
        return Rect {
            x: left_x,
            y,
            width: panel_width,
            height: panel_height,
        };
    }

    // Try below
    let below_y = elem_y + elem_height + POSITIONING_GAP;
    if below_y + panel_height <= canvas_area.y + canvas_area.height {
        // Center horizontally with element, but keep within canvas
        let x = elem_x
            .saturating_sub(panel_width.saturating_sub(elem_width) / 2)
            .max(canvas_area.x)
            .min(canvas_area.x + canvas_area.width.saturating_sub(panel_width));
        return Rect {
            x,
            y: below_y,
            width: panel_width,
            height: panel_height,
        };
    }

    // Try above
    if elem_y >= canvas_area.y + panel_height + POSITIONING_GAP {
        let above_y = elem_y - panel_height - POSITIONING_GAP;
        let x = elem_x
            .saturating_sub(panel_width.saturating_sub(elem_width) / 2)
            .max(canvas_area.x)
            .min(canvas_area.x + canvas_area.width.saturating_sub(panel_width));
        return Rect {
            x,
            y: above_y,
            width: panel_width,
            height: panel_height,
        };
    }

    // Fallback: center on canvas
    Rect {
        x: canvas_area.x + (canvas_area.width.saturating_sub(panel_width)) / 2,
        y: canvas_area.y + (canvas_area.height.saturating_sub(panel_height)) / 2,
        width: panel_width,
        height: panel_height,
    }
}
