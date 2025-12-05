use crate::app::App;
use crate::components::Component;
use crate::types::{EventHandler, Panel};
use ratatui::{
    text::Line,
    widgets::Paragraph,
    Frame,
};

pub struct PropertiesPanel;

impl PropertiesPanel {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for PropertiesPanel {}

impl Component for PropertiesPanel {
    fn draw(&self, app: &App, frame: &mut Frame) {
        let Some(area) = app.layout.properties else {
            return;
        };

        let props = vec![Line::from(""), Line::from("  (no selection)")];

        let block = super::create_panel_block("[3]-Properties", Panel::Properties, app);
        let widget = Paragraph::new(props).block(block);

        frame.render_widget(widget, area);
    }
}
