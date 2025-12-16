use crate::components::Component;
use crate::events::{EventHandler, EventResult, KeyEvent, MouseEvent};
use crate::state::AppState;
use crate::tools::Tool;
use crate::types::Panel;
use crate::utils::ModalArea;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

const MODAL_WIDTH: u16 = 25;
const MODAL_HEIGHT: u16 = 13;
const KEY_DISPLAY_WIDTH: usize = 4; // "  X " format

fn get_modal_area(canvas_area: Rect) -> ModalArea {
    ModalArea::bottom_left(canvas_area, MODAL_WIDTH, MODAL_HEIGHT)
}

fn detect_tool_click(content_row: usize) -> Option<Tool> {
    Tool::all().get(content_row).copied()
}

fn is_lock_click(content_row: usize) -> bool {
    // Lock line is at: tools (N) + empty (1) + separator (1)
    content_row == Tool::all().len() + 2
}

pub struct ToolsPanel;

impl ToolsPanel {
    pub fn new() -> Self {
        Self
    }
}

impl EventHandler for ToolsPanel {
    type State = AppState;
    fn handle_key_event(&mut self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        // Only handle when tools modal is visible and active
        if !state.show_tools_modal || state.active_panel != Panel::Tools {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char(' ') => {
                state.toggle_tools_modal();
                // Switch back to canvas when closing
                if !state.show_tools_modal {
                    state.switch_panel(Panel::Canvas);
                }
                EventResult::Consumed
            }
            KeyCode::Up | KeyCode::Char('k') => {
                state.select_prev_tool();
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.select_next_tool();
                EventResult::Consumed
            }
            KeyCode::Enter => {
                // Close modal after selecting tool with Enter
                state.toggle_tools_modal();
                EventResult::Consumed
            }
            KeyCode::Tab | KeyCode::Char('x') => {
                state.toggle_tool_lock();
                EventResult::Consumed
            }
            KeyCode::Char(c) => {
                // Direct tool selection by shortcut (stays open)
                if let Some(tool) = Tool::from_key(c) {
                    state.select_tool(tool);
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_down(&mut self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        if !state.show_tools_modal {
            return EventResult::Ignored;
        }

        let modal_area = get_modal_area(state.layout.canvas);

        if !modal_area.contains(mouse_event.column, mouse_event.row) {
            return EventResult::Ignored;
        }

        state.switch_panel(Panel::Tools);

        let content_row = modal_area.content_relative_y(mouse_event.row) as usize;

        if is_lock_click(content_row) {
            state.toggle_tool_lock();
            return EventResult::Consumed;
        }

        if let Some(tool) = detect_tool_click(content_row) {
            state.select_tool(tool);
            return EventResult::Consumed;
        }

        EventResult::Ignored
    }
}

// --- RENDERING ---

fn render_empty_line() -> Line<'static> {
    Line::from("")
}

fn render_tool_line(tool: Tool, is_selected: bool) -> Line<'static> {
    let key_char = tool.key();
    let name = tool.name().to_string();

    let key_display = if key_char == '\0' {
        " ".repeat(KEY_DISPLAY_WIDTH)
    } else {
        format!("  {} ", key_char)
    };

    let (key_style, name_style, bg_style) = if is_selected {
        (
            Style::default().fg(Color::Cyan).bg(Color::DarkGray),
            Style::default().fg(Color::Yellow).bg(Color::DarkGray),
            Style::default().bg(Color::DarkGray),
        )
    } else {
        (
            Style::default().fg(Color::Cyan),
            Style::default(),
            Style::default(),
        )
    };

    // Calculate padding to fill rest of line with background
    let content_width = KEY_DISPLAY_WIDTH + name.len();
    let padding_width = (MODAL_WIDTH as usize).saturating_sub(content_width + 2); // -2 for borders

    Line::from(vec![
        Span::styled(key_display, key_style),
        Span::styled(name, name_style),
        Span::styled(" ".repeat(padding_width), bg_style),
    ])
}

fn render_lock_line(is_locked: bool) -> Line<'static> {
    let lock_text = if is_locked { "[x]" } else { "[ ]" };
    let lock_style = if is_locked {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    Line::from(vec![
        Span::styled(lock_text, lock_style),
        Span::raw(" Lock tool"),
    ])
}

impl Component for ToolsPanel {
    fn draw(&mut self, state: &AppState, frame: &mut Frame) {
        if !state.show_tools_modal {
            return;
        }

        let modal_area = get_modal_area(state.layout.canvas);
        modal_area.clear(frame);

        let mut lines = vec![render_empty_line()];

        // Render tool lines
        for &tool in Tool::all() {
            lines.push(render_tool_line(tool, state.tool.selected_tool == tool));
        }

        // Add separator and lock line
        lines.push(render_empty_line());
        let separator = "─".repeat((MODAL_WIDTH - 2) as usize); // -2 for borders
        lines.push(Line::from(separator));
        lines.push(render_lock_line(state.tool.tool_locked));

        let border_color = if state.active_panel == Panel::Tools {
            Color::Green
        } else {
            Color::DarkGray
        };

        let block = Block::default()
            .title("Tools")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        let widget = Paragraph::new(lines).block(block);
        frame.render_widget(widget, modal_area.rect());
    }
}
