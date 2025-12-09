mod numeric;

pub use numeric::NumericInput;

use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use ratatui::text::Line;

/// Trait for property input components
pub trait PropertyInput {
    /// Render this input as a line
    fn render_line(&self) -> Line<'static>;

    /// Check if this input is focused
    fn is_focused(&self) -> bool;

    /// Set focus state
    fn set_focused(&mut self, focused: bool);

    /// Check if this input is being edited
    fn is_editing(&self) -> bool;

    /// Get the property name this input is editing
    fn property_name(&self) -> &str;

    /// Get the current value as a PropertyValue
    fn get_value(&self) -> PropertyValue;

    /// Set the value from a PropertyValue
    fn set_value(&mut self, value: PropertyValue);

    /// Handle key event with callback for value changes
    /// The callback is called with (property_name, new_value) when the value changes
    fn handle_key_event(
        &mut self,
        key: &KeyEvent,
        on_change: &mut dyn FnMut(&str, PropertyValue),
    ) -> EventResult;
}
