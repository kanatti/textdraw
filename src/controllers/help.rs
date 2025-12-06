use crate::app::HelpState;
use crate::components::HelpModal;
use crate::ui::UILayout;

/// Toggle help modal visibility
pub fn toggle(help: &mut HelpState) {
    help.show = !help.show;
    if help.show {
        help.scroll = 0; // Reset scroll when opening
    }
}

/// Scroll help modal up
pub fn scroll_up(help: &mut HelpState) {
    help.scroll = help.scroll.saturating_sub(1);
}

/// Scroll help modal down
pub fn scroll_down(help: &mut HelpState, layout: &UILayout) {
    // Calculate max scroll based on terminal height (60% for modal)
    let terminal_height = layout.canvas.map(|r| r.height).unwrap_or(40);
    let modal_height = (terminal_height * 60) / 100;
    let max_scroll = HelpModal::max_scroll(modal_height);
    help.scroll = help.scroll.saturating_add(1).min(max_scroll);
}
