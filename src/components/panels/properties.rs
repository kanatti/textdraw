use crate::components::{ChoiceInput, Component, NumericInput, PropertyInput};
use crate::elements::{Element, FieldType, PropertiesSpec, PropertyValue};
use crate::events::{EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::state::AppState;
use crate::types::Panel;
use crate::ui;
use crate::utils::ModalArea;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
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
    focused_input_index: Option<usize>,
    // Element ID we're currently editing
    current_element_id: Option<usize>,
}

impl PropertiesPanel {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            focused_input_index: None,
            current_element_id: None,
        }
    }

    /// Initialize inputs for the given element
    fn initialize(&mut self, element: &Element) {
        // Only reinitialize if element changed
        if self.current_element_id == Some(element.id()) {
            // Same element, no need to recreate inputs
            return;
        }

        // Clear existing inputs
        self.inputs.clear();
        self.focused_input_index = None;
        self.current_element_id = Some(element.id());

        // Get the properties spec from element
        let spec = element.properties_spec();

        // Create input for each field
        for field in spec.all_fields() {
            match &field.field_type {
                FieldType::Numeric { min, max } => {
                    let input = NumericInput::new(&field.name, &field.label, *min, *max);
                    self.inputs.push(Box::new(input));
                }
                FieldType::Choice { options } => {
                    let input = ChoiceInput::new(&field.name, &field.label, options.clone());
                    self.inputs.push(Box::new(input));
                }
                _ => {
                    // Other field types not yet supported
                }
            }
        }

        // Focus first input by default
        if !self.inputs.is_empty() {
            self.set_focus(0);
        }
    }

    /// Set focus to a specific input index
    fn set_focus(&mut self, index: usize) {
        if index >= self.inputs.len() {
            return;
        }
        self.focused_input_index = Some(index);
        self.inputs[index].set_focused(true);
    }

    /// Update focus from current to new index
    fn update_focus(&mut self, new_index: usize) {
        if self.inputs.is_empty() {
            return;
        }

        // Clear current focus
        if let Some(idx) = self.focused_input_index {
            self.inputs[idx].set_focused(false);
        }

        // Set new focus
        self.set_focus(new_index);
    }

    /// Move focus to next input
    fn focus_next(&mut self) {
        if self.inputs.is_empty() {
            return;
        }

        let next = match self.focused_input_index {
            Some(idx) => (idx + 1) % self.inputs.len(),
            None => 0,
        };

        self.update_focus(next);
    }

    /// Move focus to previous input
    fn focus_prev(&mut self) {
        if self.inputs.is_empty() {
            return;
        }

        let prev = match self.focused_input_index {
            Some(idx) if idx == 0 => self.inputs.len() - 1,
            Some(idx) => idx - 1,
            None => 0,
        };

        self.update_focus(prev);
    }

    /// Calculate modal position at bottom-left corner
    fn calculate_modal_area(canvas_area: Rect, content_height: u16) -> ModalArea {
        // Add 2 for top and bottom borders
        let total_height = content_height + 2;
        ModalArea::bottom_left(canvas_area, PROPERTIES_PANEL_WIDTH, total_height)
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

    /// Get the selected element if properties panel should be visible
    fn get_selected_element(state: &AppState) -> Option<(usize, &Element)> {
        // Don't show properties when in edit table mode
        if state.is_editing_table() {
            return None;
        }

        // Only show when properties visible and exactly one element selected
        if !state.show_properties || state.selection_state.selected_ids.len() != 1 {
            return None;
        }

        let element_id = state.selection_state.selected_ids[0];
        state
            .canvas
            .get_element(element_id)
            .map(|e| (element_id, e))
    }

    /// Forward key event to focused input with callback
    fn forward_to_input(
        &mut self,
        idx: usize,
        key: &KeyEvent,
        element_id: usize,
        state: &mut AppState,
    ) -> EventResult {
        let prop_name = self.inputs[idx].property_name();

        // Get current value
        let current_value = state
            .canvas
            .get_element(element_id)
            .and_then(|e| e.get_property(prop_name))
            .unwrap_or(PropertyValue::Numeric(0));

        self.inputs[idx].handle_key_event(key, &current_value, &mut |prop_name, value| {
            // Callback: update the element when value changes
            if let Some(element) = state.canvas.get_element_mut(element_id) {
                let _ = element.set_property(prop_name, value);
            }
        })
    }
}

impl EventHandler for PropertiesPanel {
    type State = AppState;

    fn handle_key_event(&mut self, state: &mut AppState, key: &KeyEvent) -> EventResult {
        // Only handle when properties panel is active
        if state.active_panel != Panel::Properties {
            return EventResult::Ignored;
        }

        // Get selected element
        let Some((element_id, element)) = Self::get_selected_element(state) else {
            return EventResult::Ignored;
        };

        // Initialize inputs if needed
        self.initialize(element);

        if self.inputs.is_empty() {
            return EventResult::Ignored;
        }

        // Check if any input is editing
        let is_editing = self
            .focused_input_index
            .map(|idx| self.inputs[idx].is_editing())
            .unwrap_or(false);

        // If editing, forward to focused input with callback
        if is_editing {
            if let Some(idx) = self.focused_input_index {
                return self.forward_to_input(idx, key, element_id, state);
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
                if let Some(idx) = self.focused_input_index {
                    return self.forward_to_input(idx, key, element_id, state);
                }
                EventResult::Ignored
            }
            // Consume Left/Right to prevent element movement when properties panel is open
            KeyCode::Left | KeyCode::Right => EventResult::Consumed,
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        // Get selected element to check if properties should be visible
        let Some((element_id, element)) = Self::get_selected_element(state) else {
            return EventResult::Ignored;
        };

        // Initialize inputs if needed
        self.initialize(element);

        // Calculate properties area
        let spec = element.properties_spec();
        let content_height = self.calculate_content_height(&spec);
        let canvas_area = state.layout.canvas;
        let area = Self::calculate_modal_area(canvas_area, content_height);

        // Check if click is inside properties panel
        if area.contains(mouse_event.column, mouse_event.row) {
            // Activate properties panel when clicked
            state.switch_panel(Panel::Properties);
            EventResult::Consumed
        } else {
            EventResult::Ignored
        }
    }
}

impl Component for PropertiesPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        // Get selected element
        let Some((element_id, element)) = Self::get_selected_element(state) else {
            return;
        };

        // Initialize inputs if needed
        self.initialize(element);

        // Get properties spec to calculate height
        let spec = element.properties_spec();
        let content_height = self.calculate_content_height(&spec);

        // Calculate dynamic position at bottom-left corner
        let canvas_area = state.layout.canvas;
        let area = Self::calculate_modal_area(canvas_area, content_height);

        area.clear(frame);

        // Build content lines
        let mut lines = vec![ui::blank_line()];

        // Element type and name
        lines.push(ui::label_value_line("Type", element.type_name()));
        lines.push(ui::label_value_line("Name", element.name().to_string()));

        lines.push(ui::blank_line());

        // Render editable properties (if any)
        let mut field_index = 0;
        let panel_active = state.active_panel == Panel::Properties;
        for section in &spec.sections {
            lines.push(ui::section_header(&section.title));

            // Render inputs for this section
            for _field in &section.fields {
                if field_index < self.inputs.len() {
                    let prop_name = self.inputs[field_index].property_name();
                    let current_value = element
                        .get_property(prop_name)
                        .unwrap_or(PropertyValue::Numeric(0));
                    lines.push(self.inputs[field_index].render_line(&current_value, panel_active));
                    field_index += 1;
                }
            }

            lines.push(ui::blank_line());
        }

        // Add shortcut helper at the bottom
        lines.push(Line::from(vec![
            ui::padded_span("Toggle: ", ui::muted_style()),
            Span::styled("p", ui::muted_style()),
            ui::padded_span("Edit: ", ui::muted_style()),
            Span::styled("Enter", ui::muted_style()),
        ]));

        let block = ui::panel_block(" Properties ", state.active_panel == Panel::Properties);
        let properties = Paragraph::new(lines).block(block);

        frame.render_widget(properties, area.rect());
    }
}
