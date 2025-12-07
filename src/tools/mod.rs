mod arrow;
mod line;
mod rectangle;
mod text;

// Re-export tool implementations
pub use arrow::ArrowTool;
pub use line::LineTool;
pub use rectangle::RectangleTool;
pub use text::TextTool;

use crate::events::EventHandler;
use crate::state::CanvasState;

/// Macro to define the Tool enum with associated names and keyboard shortcuts.
///
/// This generates:
/// - The Tool enum with all variants
/// - `all()` - returns all tools as a Vec
/// - `name()` - returns the display name for the tool
/// - `key()` - returns the keyboard shortcut for the tool
macro_rules! define_tools_enum {
    ( $( $variant:ident => ($name:expr, $key:expr) ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Tool {
            $($variant),*
        }

        impl Tool {
            pub fn all() -> Vec<Tool> {
                vec![$(Tool::$variant),*]
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(Tool::$variant => $name),*
                }
            }

            pub fn key(&self) -> char {
                match self {
                    $(Tool::$variant => $key),*
                }
            }

            pub fn from_key(c: char) -> Option<Self> {
                match c {
                    $($key => Some(Tool::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

// Define all tools as enum with their names and keys
define_tools_enum! {
    Select    => ("Select", 's'),
    Line      => ("Line", 'l'),
    Rectangle => ("Rectangle", 'r'),
    Arrow     => ("Arrow", 'a'),
    Text      => ("Text", 't'),
}

/// Trait for all drawing tools - extends EventHandler for event routing
pub trait DrawingTool: EventHandler<State = CanvasState> {
    /// Get preview points for rendering during drag (x, y, char)
    fn preview_points(&self) -> Vec<(i32, i32, char)> {
        vec![]
    }

    /// Finish current drawing operation programmatically (for tools like Text that finish on Enter)
    fn finish(&mut self, state: &mut CanvasState);

    /// Cancel current drawing operation
    fn cancel(&mut self);

    /// Check if tool is currently in a drawing state
    fn is_drawing(&self) -> bool;

    /// Downcast helper for accessing specific tool implementations
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
