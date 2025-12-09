use crate::components::inputs::PropertyInput;
use crate::elements::PropertyValue;
use crate::events::{EventResult, KeyEvent};
use crossterm::event::KeyCode;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// A choice input component for selecting from a list of options
pub struct ChoiceInput {
    // Current state
    current_value: String,
    original_value: String, // For reverting on Esc
    selected_index: usize,

    // Options
    options: Vec<String>,

    // Property identification
    property_name: String,
    label: String,

    // Edit state
    is_focused: bool,
    is_editing: bool,
}

impl ChoiceInput {
    pub fn new(
        property_name: impl Into<String>,
        label: impl Into<String>,
        initial_value: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        let initial_value = initial_value.into();
        let selected_index = options
            .iter()
            .position(|opt| opt == &initial_value)
            .unwrap_or(0);

        Self {
            current_value: initial_value.clone(),
            original_value: initial_value,
            selected_index,
            options,
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

    /// Start editing mode
    fn start_editing(&mut self) {
        if !self.is_focused {
            return;
        }
        self.is_editing = true;
        self.original_value = self.current_value.clone();
    }

    /// Cancel editing and revert to original value
    fn cancel_editing(&mut self) {
        self.is_editing = false;
        self.current_value = self.original_value.clone();
        // Restore selected_index to match original value
        self.selected_index = self
            .options
            .iter()
            .position(|opt| opt == &self.current_value)
            .unwrap_or(0);
    }

    /// Apply the selected value
    fn apply_edit(&mut self) {
        self.is_editing = false;
        // Update current_value from selected_index
        if let Some(value) = self.options.get(self.selected_index) {
            self.current_value = value.clone();
        }
    }

    /// Move to next option (with wraparound), returns true if changed
    fn cycle_next(&mut self) -> bool {
        if self.options.is_empty() {
            return false;
        }
        let old_index = self.selected_index;
        self.selected_index = (self.selected_index + 1) % self.options.len();
        if let Some(value) = self.options.get(self.selected_index) {
            self.current_value = value.clone();
        }
        old_index != self.selected_index
    }

    /// Move to previous option (with wraparound), returns true if changed
    fn cycle_prev(&mut self) -> bool {
        if self.options.is_empty() {
            return false;
        }
        let old_index = self.selected_index;
        self.selected_index = if self.selected_index == 0 {
            self.options.len() - 1
        } else {
            self.selected_index - 1
        };
        if let Some(value) = self.options.get(self.selected_index) {
            self.current_value = value.clone();
        }
        old_index != self.selected_index
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
            format!("< {} >", self.current_value)
        } else {
            self.current_value.clone()
        };

        Line::from(vec![
            Span::styled(format!("  {}: ", self.label), label_style),
            Span::styled(display_value, value_style),
            // Add padding to fill the rest of the line
            Span::styled(" ".repeat(20), bg_style),
        ])
    }
}

impl PropertyInput for ChoiceInput {
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
                    self.apply_edit();
                    EventResult::Consumed
                }
                KeyCode::Left => {
                    if self.cycle_prev() {
                        // Value changed, notify via callback immediately
                        on_change(
                            &self.property_name,
                            PropertyValue::Choice(self.current_value.clone()),
                        );
                    }
                    EventResult::Consumed
                }
                KeyCode::Right => {
                    if self.cycle_next() {
                        // Value changed, notify via callback immediately
                        on_change(
                            &self.property_name,
                            PropertyValue::Choice(self.current_value.clone()),
                        );
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
        PropertyValue::Choice(self.current_value.clone())
    }

    fn set_value(&mut self, value: PropertyValue) {
        if let PropertyValue::Choice(s) = value {
            self.current_value = s.clone();
            self.selected_index = self
                .options
                .iter()
                .position(|opt| opt == &s)
                .unwrap_or(0);
        }
    }
}
