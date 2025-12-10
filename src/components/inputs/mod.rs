mod choice;
mod numeric;

pub use choice::ChoiceInput;
pub use numeric::NumericInput;

use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use ratatui::text::Line;

/// Trait for property input components
pub trait PropertyInput {
    /// Render this input as a line with the current value
    fn render_line(&self, current_value: &PropertyValue) -> Line<'static>;

    /// Set focus state
    fn set_focused(&mut self, focused: bool);

    /// Check if this input is being edited
    fn is_editing(&self) -> bool;

    /// Get the property name this input is editing
    fn property_name(&self) -> &str;

    /// Handle key event with current value and callback for value changes
    /// The callback is called with (property_name, new_value) when the value changes
    fn handle_key_event(
        &mut self,
        key: &KeyEvent,
        current_value: &PropertyValue,
        on_change: &mut dyn FnMut(&str, PropertyValue),
    ) -> EventResult;
}
