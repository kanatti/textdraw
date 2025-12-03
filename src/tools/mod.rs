pub mod arrow;
pub mod line;
pub mod rectangle;
pub mod select;
pub mod text;

use crate::canvas::Canvas;

/// Trait for all drawing tools
pub trait DrawingTool {
    /// Called when mouse button is pressed down
    fn on_mouse_down(&mut self, x: u16, y: u16);

    /// Called during mouse drag
    fn on_mouse_drag(&mut self, x: u16, y: u16);

    /// Called when mouse button is released - commits drawing to canvas
    fn on_mouse_up(&mut self, x: u16, y: u16, canvas: &mut Canvas);

    /// Get preview points for rendering during drag (x, y, char)
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        vec![]
    }

    /// Cancel current drawing operation
    fn cancel(&mut self);

    /// Check if tool is currently in a drawing state
    fn is_drawing(&self) -> bool;

    /// Downcast helper for accessing specific tool implementations
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
