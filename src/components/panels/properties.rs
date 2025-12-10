use crate::components::{ChoiceInput, Component, NumericInput, PropertyInput};
use crate::elements::{Element, FieldType, PropertiesSpec, PropertyValue};
use crate::events::{EventHandler, EventResult, KeyEvent};
use crate::state::AppState;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

/// Panel dimensions for properties panel
const PROPERTIES_PANEL_WIDTH: u16 = 35;

/// Fixed content lines in properties panel:
/// - 1 blank line at top
/// - 2 lines for element type and name
/// - 1 blank line after header
/// - 1 line for shortcut helper at bottom
const PROPERTIES_FIXED_LINES: u16 = 5;

pub struct PropertiesPanel {
    // Input components for each property field
    inputs: Vec<Box<dyn PropertyInput>>,
    // Currently focused input index
    focused_index: Option<usize>,
    // Element ID we're currently editing
    current_element_id: Option<usize>,
}

impl PropertiesPanel {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            focused_index: None,
            current_element_id: None,
        }
    }

    /// Initialize inputs for the given element
    fn init_inputs_for_element(&mut self, element: &Element) {
        // Only reinitialize if element changed
        if self.current_element_id == Some(element.id()) {
            // Just update values
            self.update_input_values(element);
            return;
        }

        // Clear existing inputs
        self.inputs.clear();
        self.focused_index = None;
        self.current_element_id = Some(element.id());

        // Get the properties spec from element
        let spec = element.properties_spec();

        // Create input for each field
        for field in spec.all_fields() {
            let current_value = element.get_property(&field.name);

            match &field.field_type {
                FieldType::Numeric { min, max } => {
                    if let Some(PropertyValue::Numeric(value)) = current_value {
                        let input = NumericInput::new(&field.name, &field.label, value, *min, *max);
                        self.inputs.push(Box::new(input));
                    }
                }
                FieldType::Choice { options } => {
                    let value_str = if let Some(PropertyValue::Choice(s)) = current_value {
                        s
                    } else {
                        options.first().unwrap_or(&String::new()).clone()
                    };
                    let input =
                        ChoiceInput::new(&field.name, &field.label, value_str, options.clone());
                    self.inputs.push(Box::new(input));
                }
                _ => {
                    // Other field types not yet supported
                }
            }
        }

        // Focus first input by default
        if !self.inputs.is_empty() {
            self.focused_index = Some(0);
            self.inputs[0].set_focused(true);
        }
    }

    /// Update input values from element without recreating inputs
    fn update_input_values(&mut self, element: &Element) {
        for input in &mut self.inputs {
            let prop_name = input.property_name();
            if let Some(value) = element.get_property(prop_name) {
                input.set_value(value);
            }
        }
    }

    /// Move focus to next input
    fn focus_next(&mut self) {
        if self.inputs.is_empty() {
            return;
        }

        // Clear current focus
        if let Some(idx) = self.focused_index {
            self.inputs[idx].set_focused(false);
        }

        // Move to next
        let next = match self.focused_index {
            Some(idx) => (idx + 1) % self.inputs.len(),
            None => 0,
        };

        self.focused_index = Some(next);
        self.inputs[next].set_focused(true);
    }

    /// Move focus to previous input
    fn focus_prev(&mut self) {
        if self.inputs.is_empty() {
            return;
        }

        // Clear current focus
        if let Some(idx) = self.focused_index {
            self.inputs[idx].set_focused(false);
        }

        // Move to previous
        let prev = match self.focused_index {
            Some(idx) if idx == 0 => self.inputs.len() - 1,
            Some(idx) => idx - 1,
            None => 0,
        };

        self.focused_index = Some(prev);
        self.inputs[prev].set_focused(true);
    }

    /// Create a section header line
    fn section_header(text: &str) -> Line<'static> {
        Line::from(vec![Span::styled(
            format!("  {}:", text),
            Style::default().add_modifier(Modifier::BOLD),
        )])
    }

    /// Create a blank line
    fn blank_line() -> Line<'static> {
        Line::from("")
    }

    /// Calculate modal position at bottom-left corner
    fn calculate_modal_area(canvas_area: Rect, content_height: u16) -> Rect {
        // Add 2 for top and bottom borders
        let total_height = content_height + 2;

        Rect {
            x: canvas_area.x + 2,
            y: canvas_area.y + canvas_area.height.saturating_sub(total_height + 1),
            width: PROPERTIES_PANEL_WIDTH,
            height: total_height,
        }
    }

    /// Calculate the height needed for the content
    fn calculate_content_height(&self, spec: &PropertiesSpec) -> u16 {
        let mut height = PROPERTIES_FIXED_LINES;

        // For each section: header + fields + blank line
        for section in &spec.sections {
            height += 1; // section header
            height += section.fields.len() as u16; // one line per field
            height += 1; // blank line after section
        }

        height
    }
}

impl EventHandler for PropertiesPanel {
    type State = AppState;

    fn handle_key_event(&mut self, state: &mut AppState, key: &KeyEvent) -> EventResult {
        // Only handle if properties visible and exactly 1 element selected
        if !state.show_properties || state.selection_state.selected_ids.len() != 1 {
            return EventResult::Ignored;
        }

        // Get the selected element
        let element_id = state.selection_state.selected_ids[0];
        let Some(element) = state
            .canvas
            .elements()
            .iter()
            .find(|e| e.id() == element_id)
        else {
            return EventResult::Ignored;
        };

        // Initialize inputs if needed
        self.init_inputs_for_element(element);

        if self.inputs.is_empty() {
            return EventResult::Ignored;
        }

        // Check if any input is editing
        let is_editing = self
            .focused_index
            .map(|idx| self.inputs[idx].is_editing())
            .unwrap_or(false);

        // If editing, forward to focused input with callback
        if is_editing {
            if let Some(idx) = self.focused_index {
                let result = self.inputs[idx].handle_key_event(key, &mut |prop_name, value| {
                    // Callback: update the element when value changes
                    if let Some(element) = state.canvas.get_element_mut(element_id) {
                        let _ = element.set_property(prop_name, value);
                    }
                });

                return result;
            }
        }

        // Handle navigation between inputs
        match key.code {
            KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
                self.focus_next();
                EventResult::Consumed
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.focus_prev();
                EventResult::Consumed
            }
            KeyCode::Enter => {
                // Forward to focused input to start editing
                if let Some(idx) = self.focused_index {
                    return self.inputs[idx].handle_key_event(key, &mut |prop_name, value| {
                        // Callback: update the element when value changes
                        if let Some(element) = state.canvas.get_element_mut(element_id) {
                            let _ = element.set_property(prop_name, value);
                        }
                    });
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Component for PropertiesPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        if !state.show_properties {
            return;
        }

        // Only show when exactly one element is selected
        if state.selection_state.selected_ids.len() != 1 {
            return;
        }

        // Get the selected element
        let element_id = state.selection_state.selected_ids[0];
        let Some(element) = state
            .canvas
            .elements()
            .iter()
            .find(|e| e.id() == element_id)
        else {
            return;
        };

        // Initialize inputs if needed
        self.init_inputs_for_element(element);

        // Get properties spec to calculate height
        let spec = element.properties_spec();
        let content_height = self.calculate_content_height(&spec);

        // Calculate dynamic position at bottom-left corner
        let canvas_area = state.layout.canvas;
        let area = Self::calculate_modal_area(canvas_area, content_height);

        // Clear the area
        frame.render_widget(Clear, area);

        // Build content lines
        let mut lines = vec![Self::blank_line()];

        // Element type and name
        lines.push(Line::from(vec![
            Span::styled("  Type: ", Style::default().fg(Color::Yellow)),
            Span::raw(element.type_name()),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Name: ", Style::default().fg(Color::Yellow)),
            Span::raw(element.name().to_string()),
        ]));

        lines.push(Self::blank_line());

        // Render editable properties (if any)
        let spec = element.properties_spec();
        let mut field_index = 0;
        for section in &spec.sections {
            lines.push(Self::section_header(&section.title));

            // Render inputs for this section
            for _field in &section.fields {
                if field_index < self.inputs.len() {
                    lines.push(self.inputs[field_index].render_line());
                    field_index += 1;
                }
            }

            lines.push(Self::blank_line());
        }

        // Add shortcut helper at the bottom
        lines.push(Line::from(vec![
            Span::styled("  Toggle: ", Style::default().fg(Color::DarkGray)),
            Span::styled("p", Style::default().fg(Color::DarkGray)),
        ]));

        // Create the paragraph widget
        let properties = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Properties ")
                .border_style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(properties, area);
    }
}
