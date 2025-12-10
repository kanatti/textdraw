use crate::components::inputs::PropertyInput;
use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// A numeric input component with editing capabilities
pub struct NumericInput {
    // Value and constraints
    value: u16,
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
        value: u16,
        min: u16,
        max: u16,
    ) -> Self {
        Self {
            value,
            min,
            max,
            property_name: property_name.into(),
            label: label.into(),
            is_focused: false,
            is_editing: false,
            edit_buffer: String::new(),
        }
    }

    /// Set the value (clamped to min/max)
    pub fn set_value(&mut self, value: u16) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        if !focused {
            self.is_editing = false;
        }
    }

    /// Start editing mode
    fn start_editing(&mut self) {
        if !self.is_focused {
            return;
        }
        self.is_editing = true;
        self.edit_buffer = self.value.to_string();
    }

    /// Cancel editing and revert to original value
    fn cancel_editing(&mut self) {
        self.is_editing = false;
        self.edit_buffer.clear();
    }

    /// Apply the edited value
    fn apply_edit(&mut self) -> bool {
        if let Ok(new_value) = self.edit_buffer.parse::<u16>() {
            let clamped = new_value.clamp(self.min, self.max);
            if clamped != self.value {
                self.value = clamped;
                self.is_editing = false;
                self.edit_buffer.clear();
                return true; // Value changed
            }
        }
        self.is_editing = false;
        self.edit_buffer.clear();
        false // No change
    }

    /// Increment value by 1, returns true if value changed
    fn increment(&mut self) -> bool {
        if self.is_editing {
            // Increment the buffer value and apply immediately
            if let Ok(val) = self.edit_buffer.parse::<u16>() {
                let new_val = val.saturating_add(1).min(self.max);
                if new_val != val {
                    self.edit_buffer = new_val.to_string();
                    self.value = new_val;
                    return true;
                }
            }
        } else {
            let new_value = self.value.saturating_add(1).min(self.max);
            if new_value != self.value {
                self.value = new_value;
                return true;
            }
        }
        false
    }

    /// Decrement value by 1, returns true if value changed
    fn decrement(&mut self) -> bool {
        if self.is_editing {
            // Decrement the buffer value and apply immediately
            if let Ok(val) = self.edit_buffer.parse::<u16>() {
                let new_val = val.saturating_sub(1).max(self.min);
                if new_val != val {
                    self.edit_buffer = new_val.to_string();
                    self.value = new_val;
                    return true;
                }
            }
        } else {
            let new_value = self.value.saturating_sub(1).max(self.min);
            if new_value != self.value {
                self.value = new_value;
                return true;
            }
        }
        false
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

    /// Render this input as a Line
    pub fn render_line(&self) -> Line<'static> {
        let (label_style, value_style, bg_style) = if self.is_editing {
            // Editing: dark gray background with cyan text
            (
                Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                Style::default().fg(Color::Cyan).bg(Color::DarkGray),
                Style::default().bg(Color::DarkGray),
            )
        } else if self.is_focused {
            // Focused: dark gray background
            (
                Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                Style::default().fg(Color::White).bg(Color::DarkGray),
                Style::default().bg(Color::DarkGray),
            )
        } else {
            // Normal: yellow label, default value
            (
                Style::default().fg(Color::Yellow),
                Style::default(),
                Style::default(),
            )
        };

        let display_value = if self.is_editing {
            format!("{}▎", self.edit_buffer) // Cursor at end
        } else {
            self.value.to_string()
        };

        Line::from(vec![
            Span::styled(format!("  {}: ", self.label), label_style),
            Span::styled(display_value, value_style),
            // Add padding to fill the rest of the line
            Span::styled(" ".repeat(20), bg_style),
        ])
    }
}

impl PropertyInput for NumericInput {
    fn handle_key_event(
        &mut self,
        key: &KeyEvent,
        on_change: &mut dyn FnMut(&str, PropertyValue),
    ) -> EventResult {
        if !self.is_focused {
            return EventResult::Ignored;
        }

        if self.is_editing {
            // Handle editing mode
            match key.code {
                KeyCode::Esc => {
                    self.cancel_editing();
                    EventResult::Consumed
                }
                KeyCode::Enter => {
                    if self.apply_edit() {
                        // Value changed, notify via callback
                        on_change(&self.property_name, PropertyValue::Numeric(self.value));
                    }
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
                    if self.increment() {
                        // Value changed, notify via callback
                        on_change(&self.property_name, PropertyValue::Numeric(self.value));
                    }
                    EventResult::Consumed
                }
                KeyCode::Down => {
                    if self.decrement() {
                        // Value changed, notify via callback
                        on_change(&self.property_name, PropertyValue::Numeric(self.value));
                    }
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            }
        } else {
            // When focused but not editing, only handle Enter to start editing
            // All other keys (Up/Down/etc) are ignored so PropertiesPanel can handle navigation
            match key.code {
                KeyCode::Enter => {
                    self.start_editing();
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            }
        }
    }

    fn render_line(&self) -> Line<'static> {
        self.render_line()
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.set_focused(focused)
    }

    fn is_editing(&self) -> bool {
        self.is_editing
    }

    fn property_name(&self) -> &str {
        &self.property_name
    }

    fn get_value(&self) -> PropertyValue {
        PropertyValue::Numeric(self.value)
    }

    fn set_value(&mut self, value: PropertyValue) {
        if let PropertyValue::Numeric(n) = value {
            self.set_value(n);
        }
    }
}
