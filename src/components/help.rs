use crate::app::AppState;
use crate::components::Component;
use crate::controllers::help;
use crate::events::{EventHandler, EventResult};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};

const KEY_COLUMN_WIDTH: usize = 11;

#[derive(Debug, Clone)]
enum HelpLine {
    Title(&'static str),
    Subtitle(&'static str),
    Description(&'static str),
    Section(&'static str),
    KeyBinding {
        key: &'static str,
        desc: &'static str,
    },
    CommandHeader,
    Command {
        cmd: &'static str,
        args: &'static str,
        aliases: &'static [&'static str],
        desc: &'static str,
    },
    Blank,
}

impl HelpLine {
    fn to_line(&self) -> Line<'static> {
        match self {
            HelpLine::Title(text) => Line::from(Span::styled(
                text.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            HelpLine::Subtitle(text) => Line::from(Span::styled(
                text.to_string(),
                Style::default().fg(Color::DarkGray),
            )),
            HelpLine::Description(text) => {
                Line::from(Span::styled(text.to_string(), Style::default()))
            }
            HelpLine::Section(text) => Line::from(Span::styled(
                text.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            HelpLine::KeyBinding { key, desc } => {
                let formatted_key = format!("  {:<width$}", key, width = KEY_COLUMN_WIDTH);
                Line::from(vec![
                    Span::styled(formatted_key, Style::default().fg(Color::Cyan)),
                    Span::raw(desc.to_string()),
                ])
            }
            HelpLine::CommandHeader => {
                const CMD_COL_WIDTH: usize = 20;
                const DESC_COL_WIDTH: usize = 18;

                Line::from(vec![
                    Span::styled(
                        format!("  {:<width$}", "Command", width = CMD_COL_WIDTH - 2),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:<width$}", "Description", width = DESC_COL_WIDTH),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("Alias", Style::default().add_modifier(Modifier::BOLD)),
                ])
            }
            HelpLine::Command {
                cmd,
                args,
                aliases,
                desc,
            } => {
                const CMD_COL_WIDTH: usize = 20;
                const DESC_COL_WIDTH: usize = 18;

                let mut spans = vec![];

                // Command column
                let cmd_with_args = if args.is_empty() {
                    format!("  {}", cmd)
                } else {
                    format!("  {} {}", cmd, args)
                };

                spans.push(Span::styled(
                    format!("{:<width$}", cmd_with_args, width = CMD_COL_WIDTH),
                    Style::default().fg(Color::Yellow),
                ));

                // Description column
                spans.push(Span::raw(format!(
                    "{:<width$}",
                    desc,
                    width = DESC_COL_WIDTH
                )));

                // Alias column
                if !aliases.is_empty() {
                    spans.push(Span::styled(
                        aliases.join(", "),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                Line::from(spans)
            }
            HelpLine::Blank => Line::from(""),
        }
    }
}

const fn title(text: &'static str) -> HelpLine {
    HelpLine::Title(text)
}

const fn subtitle(text: &'static str) -> HelpLine {
    HelpLine::Subtitle(text)
}

const fn description(text: &'static str) -> HelpLine {
    HelpLine::Description(text)
}

const fn section(text: &'static str) -> HelpLine {
    HelpLine::Section(text)
}

const fn keybinding(key: &'static str, desc: &'static str) -> HelpLine {
    HelpLine::KeyBinding { key, desc }
}

const fn command_header() -> HelpLine {
    HelpLine::CommandHeader
}

const fn command(
    cmd: &'static str,
    args: &'static str,
    aliases: &'static [&'static str],
    desc: &'static str,
) -> HelpLine {
    HelpLine::Command {
        cmd,
        args,
        aliases,
        desc,
    }
}

const fn blank() -> HelpLine {
    HelpLine::Blank
}

const HELP_LINES: &[HelpLine] = &[
    subtitle("Press Esc to close"),
    blank(),
    title("TextDraw v0.1.0"),
    blank(),
    description("Interactive terminal ASCII diagram editor built with Ratatui."),
    description("Create and edit diagrams using simple drawing tools and keyboard shortcuts."),
    blank(),
    title("Keyboard Shortcuts"),
    blank(),
    section("Tools"),
    keybinding("s", "Select tool"),
    keybinding("l", "Line tool"),
    keybinding("r", "Rectangle tool"),
    keybinding("a", "Arrow tool"),
    keybinding("t", "Text tool"),
    blank(),
    section("Selection"),
    keybinding("Click", "Select element"),
    keybinding("Drag", "Select multiple elements"),
    keybinding("←↑↓→", "Move selected elements"),
    keybinding("⌫/Del", "Delete selected elements"),
    blank(),
    section("Panels"),
    keybinding("0", "Canvas"),
    keybinding("1", "Tools"),
    keybinding("2", "Elements"),
    keybinding("3", "Properties"),
    blank(),
    section("General"),
    keybinding(":", "Enter command mode"),
    keybinding("Ctrl+S", "Quick save command"),
    keybinding("Ctrl+O", "Quick open command"),
    keybinding("Esc", "Select tool / Cancel"),
    keybinding("?", "Toggle help"),
    keybinding("q", "Quit"),
    blank(),
    title("Command Mode"),
    blank(),
    description("Press : to enter command mode. Type commands to save/load files."),
    blank(),
    command_header(),
    command(":save", "<file>", &[":w", ":s"], "Save diagram"),
    command(":open", "<file>", &[":e", ":o"], "Open diagram"),
    blank(),
];

pub struct HelpModal;

impl HelpModal {
    pub fn new() -> Self {
        Self
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }

    /// Calculate maximum scroll offset based on content and viewport height
    pub fn max_scroll(viewport_height: u16) -> u16 {
        let total_lines = HELP_LINES.len() as u16;
        let visible_lines = viewport_height.saturating_sub(2); // Account for borders
        total_lines.saturating_sub(visible_lines)
    }
}

impl EventHandler for HelpModal {
    fn handle_key_event(&self, state: &mut AppState, key_event: &KeyEvent) -> EventResult {
        if !state.help.show {
            return EventResult::Ignored;
        }

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                help::scroll_up(&mut state.help);
                EventResult::Consumed
            }
            KeyCode::Down | KeyCode::Char('j') => {
                help::scroll_down(&mut state.help, &state.layout);
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse_scroll(&self, state: &mut AppState, mouse_event: &MouseEvent) -> EventResult {
        if !state.help.show {
            return EventResult::Ignored;
        }

        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                help::scroll_up(&mut state.help);
                EventResult::Consumed
            }
            MouseEventKind::ScrollDown => {
                help::scroll_down(&mut state.help, &state.layout);
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Component for HelpModal {
    fn draw(&self, state: &AppState, frame: &mut Frame) {
        if !state.help.show {
            return;
        }

        let area = Self::centered_rect(60, 60, frame.area());

        // Clear the area
        frame.render_widget(Clear, area);

        let help_text: Vec<Line> = HELP_LINES.iter().map(|line| line.to_line()).collect();

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green))
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .alignment(Alignment::Left)
            .scroll((state.help.scroll, 0));

        frame.render_widget(help, area);

        // Render scrollbar - inside the block area (not overlapping borders)
        let scrollbar_area = Rect {
            x: area.x + area.width - 1,
            y: area.y + 1,
            width: 1,
            height: area.height.saturating_sub(2),
        };

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let max_scroll = Self::max_scroll(area.height);
        let mut scrollbar_state =
            ScrollbarState::new(max_scroll as usize).position(state.help.scroll as usize);

        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}
