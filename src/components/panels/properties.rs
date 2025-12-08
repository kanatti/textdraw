use crate::components::Component;
use crate::elements::{ArrowElement, Element, LineElement, RectangleElement, TextElement};
use crate::events::EventHandler;
use crate::state::AppState;
use crate::ui::calculate_smart_position;
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

/// Panel dimensions for properties panel
const PROPERTIES_PANEL_WIDTH: u16 = 35;
const PROPERTIES_PANEL_HEIGHT: u16 = 15;

pub struct PropertiesPanel;

impl PropertiesPanel {
    pub fn new() -> Self {
        Self
    }

    fn get_element_type_name(element: &Element) -> &'static str {
        match element {
            Element::Line(_) => "Line",
            Element::Rectangle(_) => "Rectangle",
            Element::Arrow(_) => "Arrow",
            Element::Text(_) => "Text",
        }
    }

    // Semantic helper functions for consistent styling

    /// Creates a property line with label and value
    fn property_line(label: &str, value: String) -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("  {}: ", label), Style::default().fg(Color::Yellow)),
            Span::raw(value),
        ])
    }

    /// Creates a section header line (bold, no color)
    fn section_header(text: &str) -> Line<'static> {
        Line::from(vec![Span::styled(
            format!("  {}:", text),
            Style::default().add_modifier(Modifier::BOLD),
        )])
    }

    /// Creates a blank line for spacing
    fn blank_line() -> Line<'static> {
        Line::from("")
    }

    // Element-specific property display methods

    fn draw_line_properties(lines: &mut Vec<Line<'static>>, line: &LineElement) {
        lines.push(Self::section_header("Segments"));
        lines.push(Self::property_line(
            "count",
            line.segments.len().to_string(),
        ));

        if let Some(first_seg) = line.segments.first() {
            lines.push(Self::blank_line());
            lines.push(Self::section_header("First Segment"));
            lines.push(Self::property_line(
                "start_x",
                first_seg.start.x.to_string(),
            ));
            lines.push(Self::property_line(
                "start_y",
                first_seg.start.y.to_string(),
            ));
            lines.push(Self::property_line("length", first_seg.length.to_string()));
            lines.push(Self::property_line(
                "direction",
                format!("{:?}", first_seg.direction),
            ));
        }
    }

    fn draw_rectangle_properties(lines: &mut Vec<Line<'static>>, rect: &RectangleElement) {
        lines.push(Self::section_header("Position"));
        lines.push(Self::property_line("x", rect.start.x.to_string()));
        lines.push(Self::property_line("y", rect.start.y.to_string()));

        lines.push(Self::blank_line());

        lines.push(Self::section_header("Size"));
        lines.push(Self::property_line("width", rect.width.to_string()));
        lines.push(Self::property_line("height", rect.height.to_string()));
    }

    fn draw_arrow_properties(lines: &mut Vec<Line<'static>>, arrow: &ArrowElement) {
        lines.push(Self::section_header("Segments"));
        lines.push(Self::property_line(
            "count",
            arrow.segments.len().to_string(),
        ));

        if let Some(first_seg) = arrow.segments.first() {
            lines.push(Self::blank_line());
            lines.push(Self::section_header("First Segment"));
            lines.push(Self::property_line(
                "start_x",
                first_seg.start.x.to_string(),
            ));
            lines.push(Self::property_line(
                "start_y",
                first_seg.start.y.to_string(),
            ));
            lines.push(Self::property_line("length", first_seg.length.to_string()));
            lines.push(Self::property_line(
                "direction",
                format!("{:?}", first_seg.direction),
            ));
        }
    }

    fn draw_text_properties(lines: &mut Vec<Line<'static>>, text: &TextElement) {
        lines.push(Self::section_header("Position"));
        lines.push(Self::property_line("x", text.position.x.to_string()));
        lines.push(Self::property_line("y", text.position.y.to_string()));

        lines.push(Self::blank_line());

        lines.push(Self::section_header("Content"));
        lines.push(Self::property_line("text", text.text.clone()));
    }
}

impl EventHandler for PropertiesPanel {
    type State = AppState;
}

impl Component for PropertiesPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        // Only show when exactly one element is selected and properties are not toggled off
        if state.selection_state.selected_ids.len() != 1 || !state.show_properties {
            return;
        }

        let element_id = state.selection_state.selected_ids[0];

        let Some(element) = state
            .canvas
            .elements()
            .iter()
            .find(|e| e.id() == element_id)
        else {
            return; // Element not found
        };

        // Get element bounds (canvas coordinates)
        let bounds = element.bounds();
        let elem_canvas_x = bounds.min.x;
        let elem_canvas_y = bounds.min.y;
        let elem_width = bounds.max.x.saturating_sub(bounds.min.x);
        let elem_height = bounds.max.y.saturating_sub(bounds.min.y);

        // Convert to screen coordinates
        let canvas_area = state.layout.canvas;
        let elem_screen_x = canvas_area.x + 1 + elem_canvas_x; // +1 for border
        let elem_screen_y = canvas_area.y + 1 + elem_canvas_y; // +1 for border

        // Calculate smart position for the floating panel
        let area = calculate_smart_position(
            elem_screen_x,
            elem_screen_y,
            elem_width,
            elem_height,
            canvas_area,
            PROPERTIES_PANEL_WIDTH,
            PROPERTIES_PANEL_HEIGHT,
        );

        // Build the property lines
        let mut lines = vec![Self::blank_line()];

        // Common properties
        let element_type = Self::get_element_type_name(element);
        lines.push(Self::property_line("Type", element_type.to_string()));
        lines.push(Self::property_line("Name", element.name().to_string()));

        lines.push(Self::blank_line());

        // Element-specific properties
        match element {
            Element::Line(line) => Self::draw_line_properties(&mut lines, line),
            Element::Rectangle(rect) => Self::draw_rectangle_properties(&mut lines, rect),
            Element::Arrow(arrow) => Self::draw_arrow_properties(&mut lines, arrow),
            Element::Text(text) => Self::draw_text_properties(&mut lines, text),
        }

        // Clear the background and render as floating overlay
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Properties (toggle: p)")
            .border_style(Style::default().fg(Color::Yellow));

        let widget = Paragraph::new(lines).block(block);
        frame.render_widget(widget, area);
    }
}
