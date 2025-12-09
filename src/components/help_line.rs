use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

const KEY_COLUMN_WIDTH: usize = 11;

#[derive(Debug, Clone)]
pub enum HelpLine {
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
    /// Convert a HelpLine to a ratatui Line for rendering
    pub fn to_line(&self) -> Line<'static> {
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

pub const HELP_LINES: &[HelpLine] = &[
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
    section("General"),
    keybinding("Space", "Toggle tools modal"),
    keybinding("p", "Toggle properties"),
    keybinding(":", "Enter command mode"),
    keybinding("Ctrl+S", "Quick save command"),
    keybinding("Ctrl+O", "Quick open command"),
    keybinding("Esc", "Cancel"),
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
