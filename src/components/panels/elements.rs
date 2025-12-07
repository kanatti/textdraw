use crate::state::AppState;
use crate::components::Component;
use crate::events::EventHandler;
use crate::types::Panel;
use ratatui::{Frame, text::Line, widgets::Paragraph};

pub struct ElementsPanel;

impl ElementsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for ElementsPanel {}

impl Component for ElementsPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        let Some(area) = state.layout.elements else {
            return;
        };

        let elements = vec![Line::from(""), Line::from("  (empty)")];

        let block = super::create_panel_block("[2]-Elements", Panel::Elements, state);
        let widget = Paragraph::new(elements).block(block);

        frame.render_widget(widget, area);
    }
}
