use crate::components::HelpModal;
use crate::ui::UILayout;

pub struct HelpState {
    pub show: bool,
    pub scroll: u16,
}

impl HelpState {
    pub fn new() -> Self {
        Self {
            show: false,
            scroll: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.show = !self.show;
        if self.show {
            self.scroll = 0; // Reset scroll when opening
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, layout: &UILayout) {
        // Calculate max scroll based on terminal height (60% for modal)
        let terminal_height = layout.canvas.map(|r| r.height).unwrap_or(40);
        let modal_height = (terminal_height * 60) / 100;
        let max_scroll = HelpModal::max_scroll(modal_height);
        self.scroll = self.scroll.saturating_add(1).min(max_scroll);
    }
}
