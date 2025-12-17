use crate::components::inputs::PropertyInput;
use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use crate::ui;
use crossterm::event::KeyCode;
use ratatui::text::Line;

/// A numeric input component with editing capabilities
pub struct NumericInput {
    // Constraints
    min: u16,
    max: u16,

    // Property identification
    property_name: String,
    label: String,

    // Edit state
    is_focused: bool,
    is_editing: bool,
    edit_buffer: String,
}

impl NumericInput {
    pub fn new(
        property_name: impl Into<String>,
        label: impl Into<String>,
        min: u16,
        max: u16,
    ) -> Self {
        Self {
            min,
            max,
            property_name: property_name.into(),
            label: label.into(),
            is_focused: false,
            is_editing: false,
            edit_buffer: String::new(),
        }
    }

    /// Adjust value while editing with given operation, returns the new value if changed
    fn adjust_value(&mut self, op: impl Fn(u16) -> u16) -> Option<u16> {
        // Parse buffer, fallback to min if empty
        let base_value = self.edit_buffer.parse::<u16>().unwrap_or(self.min);
        let new_value = op(base_value);

        if new_value != base_value {
            self.edit_buffer = new_value.to_string();
            Some(new_value)
        } else {
            None
        }
    }

    /// Increment value by 1 while editing, returns the new value if changed
    fn increment(&mut self) -> Option<u16> {
        let max = self.max;
        self.adjust_value(|v| v.saturating_add(1).min(max))
    }

    /// Decrement value by 1 while editing, returns the new value if changed
    fn decrement(&mut self) -> Option<u16> {
        let min = self.min;
        self.adjust_value(|v| v.saturating_sub(1).max(min))
    }

    /// Append a digit character to the buffer
    fn insert_char(&mut self, c: char) {
        if self.edit_buffer.len() < 5 && c.is_ascii_digit() {
            self.edit_buffer.push(c);
        }
    }

    /// Delete last character from the buffer (backspace)
    fn delete_last_char(&mut self) {
        self.edit_buffer.pop();
    }

    fn exit_editing(&mut self) {
        self.is_editing = false;
        self.edit_buffer.clear();
    }
}

impl PropertyInput for NumericInput {
    fn render_line(&self, current_value: &PropertyValue, panel_active: bool) -> Line<'static> {
        let value = match current_value {
            PropertyValue::Numeric(n) => *n,
            _ => 0, // Fallback, shouldn't happen
        };

        let styles = ui::input_styles(self.is_editing, self.is_focused, panel_active);

        let display_value = if self.is_editing {
            format!("{}▎", self.edit_buffer) // Cursor at end
        } else {
            value.to_string()
        };

        ui::input_line(&self.label, display_value, styles)
    }

    fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        if !focused {
            self.exit_editing();
        }
    }

    fn is_editing(&self) -> bool {
        self.is_editing
    }

    fn property_name(&self) -> &str {
        &self.property_name
    }

    fn handle_key_event(
        &mut self,
        key: &KeyEvent,
        current_value: &PropertyValue,
        on_change: &mut dyn FnMut(&str, PropertyValue),
    ) -> EventResult {
        if !self.is_focused {
            return EventResult::Ignored;
        }

        let value = match current_value {
            PropertyValue::Numeric(n) => *n,
            _ => return EventResult::Ignored,
        };

        if self.is_editing {
            // Handle editing mode
            match key.code {
                KeyCode::Esc => {
                    self.exit_editing();
                    EventResult::Consumed
                }
                KeyCode::Enter => {
                    // Parse, clamp, and call callback
                    if let Ok(new_value) = self.edit_buffer.parse::<u16>() {
                        let clamped = new_value.clamp(self.min, self.max);
                        on_change(&self.property_name, PropertyValue::Numeric(clamped));
                    }
                    self.exit_editing();
                    EventResult::Consumed
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    self.insert_char(c);
                    EventResult::Consumed
                }
                KeyCode::Backspace => {
                    self.delete_last_char();
                    EventResult::Consumed
                }
                KeyCode::Up => {
                    if let Some(new_value) = self.increment() {
                        // Value changed, notify via callback
                        on_change(&self.property_name, PropertyValue::Numeric(new_value));
                    }
                    EventResult::Consumed
                }
                KeyCode::Down => {
                    if let Some(new_value) = self.decrement() {
                        // Value changed, notify via callback
                        on_change(&self.property_name, PropertyValue::Numeric(new_value));
                    }
                    EventResult::Consumed
                }
                _ => {
                    // Consume all other keys while editing to prevent leaking to parent handlers
                    EventResult::Consumed
                }
            }
        } else {
            // When focused but not editing, only handle Enter to start editing
            // All other keys (Up/Down/etc) are ignored so PropertiesPanel can handle navigation
            match key.code {
                KeyCode::Enter => {
                    self.is_editing = true;
                    self.edit_buffer = value.to_string();
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            }
        }
    }
}
