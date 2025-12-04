use crate::app::App;
use crate::component::Component;
use crate::types::Panel;
use ratatui::{
    layout::Rect,
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

impl Component for PropertiesPanel {
    fn draw(&self, app: &App, frame: &mut Frame, area: Rect) {
        let props = vec![Line::from(""), Line::from("  (no selection)")];

        let block = super::create_panel_block("[3]-Properties", Panel::Properties, app.active_panel);
        let widget = Paragraph::new(props).block(block);

        frame.render_widget(widget, area);
    }
}
