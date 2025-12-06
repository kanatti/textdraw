use crate::components::HelpModal;

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

    pub fn scroll_down(&mut self, modal_height: u16) {
        let max_scroll = HelpModal::max_scroll(modal_height);
        self.scroll = self.scroll.saturating_add(1).min(max_scroll);
    }
}
