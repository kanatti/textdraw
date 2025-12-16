use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Clear;

/// Standard border width for modals
const BORDER: u16 = 1;

/// Standard padding (empty line) inside modal border
const PADDING: u16 = 1;

/// Calculate rect for a modal at the bottom-left corner
///
/// Uses standard margins: 2px from left, 1px from bottom
fn bottom_left_rect(canvas_area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: canvas_area.x + 2,
        y: canvas_area
            .y
            .saturating_add(canvas_area.height)
            .saturating_sub(height)
            .saturating_sub(1),
        width,
        height,
    }
}

/// Calculate rect for a centered modal with percentage-based sizing
fn centered_percent_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// A wrapper around Rect that provides modal-specific utility methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModalArea {
    rect: Rect,
}

impl ModalArea {
    /// Create a new ModalArea from a Rect
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    /// Position a modal at the bottom-left corner
    ///
    /// Uses standard margins: 2px from left, 1px from bottom
    ///
    /// # Arguments
    /// * `canvas_area` - The canvas area to position relative to
    /// * `width` - Width of the modal
    /// * `height` - Height of the modal (including borders)
    pub fn bottom_left(canvas_area: Rect, width: u16, height: u16) -> Self {
        Self::new(bottom_left_rect(canvas_area, width, height))
    }

    /// Position a modal at the center with percentage-based sizing
    ///
    /// # Arguments
    /// * `area` - The area to center within
    /// * `percent_x` - Horizontal percentage of the area (0-100)
    /// * `percent_y` - Vertical percentage of the area (0-100)
    pub fn centered_percent(area: Rect, percent_x: u16, percent_y: u16) -> Self {
        Self::new(centered_percent_rect(area, percent_x, percent_y))
    }

    /// Get the underlying Rect
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Check if a point (x, y) is inside the modal area
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.rect.x
            && x < self.rect.x + self.rect.width
            && y >= self.rect.y
            && y < self.rect.y + self.rect.height
    }

    /// Calculate relative Y position from the top of the modal content area
    ///
    /// This accounts for border (1px) and padding (1px empty line), so the returned
    /// position is 0-indexed from the first line of actual content.
    ///
    /// # Arguments
    /// * `y` - Absolute Y coordinate
    pub fn content_relative_y(&self, y: u16) -> u16 {
        y.saturating_sub(self.rect.y + BORDER + PADDING)
    }

    /// Clear the modal area
    pub fn clear(&self, frame: &mut Frame) {
        frame.render_widget(Clear, self.rect);
    }
}

impl From<Rect> for ModalArea {
    fn from(rect: Rect) -> Self {
        Self::new(rect)
    }
}
