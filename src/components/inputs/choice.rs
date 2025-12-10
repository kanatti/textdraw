use crate::components::inputs::PropertyInput;
use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use crate::styles;
use crossterm::event::KeyCode;
use ratatui::text::{Line, Span};

/// A choice input component for selecting from a list of options
pub struct ChoiceInput {
    // Edit state
    selected_index: usize,
    original_index: usize, // For reverting on Esc
    is_focused: bool,
    is_editing: bool,

    // Options
    options: Vec<String>,
    max_option_width: usize, // Width of longest option

    // Property identification
    property_name: String,
    label: String,
}

impl ChoiceInput {
    pub fn new(
        property_name: impl Into<String>,
        label: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        // Calculate max width for fixed-width display
        let max_option_width = options.iter().map(|s| s.len()).max().unwrap_or(0);

        Self {
            selected_index: 0,
            original_index: 0,
            options,
            max_option_width,
            property_name: property_name.into(),
            label: label.into(),
            is_focused: false,
            is_editing: false,
        }
    }

    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        if !focused {
            self.is_editing = false;
        }
    }

    /// Cancel editing and revert to original selection
    fn cancel_editing(&mut self) {
        self.is_editing = false;
        self.selected_index = self.original_index;
    }

    /// Move to next option (with wraparound), returns the new value
    fn cycle_next(&mut self) -> Option<String> {
        if self.options.is_empty() {
            return None;
        }
        self.selected_index = (self.selected_index + 1) % self.options.len();
        self.options.get(self.selected_index).cloned()
    }

    /// Move to previous option (with wraparound), returns the new value
    fn cycle_prev(&mut self) -> Option<String> {
        if self.options.is_empty() {
            return None;
        }
        self.selected_index = if self.selected_index == 0 {
            self.options.len() - 1
        } else {
            self.selected_index - 1
        };
        self.options.get(self.selected_index).cloned()
    }

    /// Render this input as a Line
    fn render_line_internal(&self, current_value: &str) -> Line<'static> {
        let styles = styles::input_styles(self.is_editing, self.is_focused);

        let display_value = if self.is_editing {
            // Show currently selected option while editing
            let selected_value = self
                .options
                .get(self.selected_index)
                .map(|s| s.as_str())
                .unwrap_or("");
            // Use fixed width based on longest option to prevent shifting
            format!(
                "< {:^width$} >",
                selected_value,
                width = self.max_option_width
            )
        } else {
            current_value.to_string()
        };

        Line::from(vec![
            Span::styled(format!("  {}: ", self.label), styles.label),
            Span::styled(display_value, styles.value),
            // Add padding to fill the rest of the line
            Span::styled(" ".repeat(20), styles.background),
        ])
    }
}

impl PropertyInput for ChoiceInput {
    fn render_line(&self, current_value: &PropertyValue) -> Line<'static> {
        let value = match current_value {
            PropertyValue::Choice(s) => s.as_str(),
            _ => "", // Fallback, shouldn't happen
        };
        self.render_line_internal(value)
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
            PropertyValue::Choice(s) => s.as_str(),
            _ => return EventResult::Ignored,
        };

        if self.is_editing {
            // Handle editing mode
            match key.code {
                KeyCode::Esc => {
                    self.cancel_editing();
                    EventResult::Consumed
                }
                KeyCode::Enter => {
                    // Get selected value and call callback
                    if let Some(new_value) = self.options.get(self.selected_index) {
                        on_change(
                            &self.property_name,
                            PropertyValue::Choice(new_value.clone()),
                        );
                    }
                    self.is_editing = false;
                    EventResult::Consumed
                }
                KeyCode::Left => {
                    if let Some(new_value) = self.cycle_prev() {
                        // Value changed, notify via callback immediately
                        on_change(&self.property_name, PropertyValue::Choice(new_value));
                    }
                    EventResult::Consumed
                }
                KeyCode::Right => {
                    if let Some(new_value) = self.cycle_next() {
                        // Value changed, notify via callback immediately
                        on_change(&self.property_name, PropertyValue::Choice(new_value));
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
                    // Set selected_index to match current value
                    self.selected_index = self
                        .options
                        .iter()
                        .position(|opt| opt == value)
                        .unwrap_or(0);
                    self.original_index = self.selected_index;
                    EventResult::Consumed
                }
                _ => EventResult::Ignored,
            }
        }
    }
}
