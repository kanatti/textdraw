use crate::app::App;
use crate::component::Component;
use crate::types::Panel;
use ratatui::{
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub struct ElementsPanel;

impl ElementsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ElementsPanel {
    fn draw(&self, app: &App, frame: &mut Frame) {
        let Some(area) = app.layout.elements else {
            return;
        };

        let elements = vec![Line::from(""), Line::from("  (empty)")];

        let block = super::create_panel_block("[2]-Elements", Panel::Elements, app.active_panel);
        let widget = Paragraph::new(elements).block(block);

        frame.render_widget(widget, area);
    }
}
