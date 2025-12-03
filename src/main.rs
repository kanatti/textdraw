use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseEventKind, EnableMouseCapture, DisableMouseCapture};
use crossterm::execute;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    // Enable mouse capture
    execute!(std::io::stdout(), EnableMouseCapture)?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    // Disable mouse capture
    execute!(std::io::stdout(), DisableMouseCapture)?;

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            app.canvas_area = render(frame, &app);
        })?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('0') => app.active_panel = Panel::Canvas,
                KeyCode::Char('1') => app.active_panel = Panel::Tools,
                KeyCode::Char('2') => app.active_panel = Panel::Elements,
                KeyCode::Char('3') => app.active_panel = Panel::Properties,
                _ => {}
            },
            Event::Mouse(mouse) => {
                // Only track mouse when Canvas panel is active
                if app.active_panel == Panel::Canvas {
                    if let MouseEventKind::Moved = mouse.kind {
                        // Convert screen coordinates to canvas coordinates
                        if let Some(canvas_area) = app.canvas_area {
                            let canvas_x = mouse.column.saturating_sub(canvas_area.x + 1);
                            let canvas_y = mouse.row.saturating_sub(canvas_area.y + 1);

                            // Check if mouse is within canvas bounds
                            if canvas_x < canvas_area.width.saturating_sub(2)
                                && canvas_y < canvas_area.height.saturating_sub(2) {
                                app.cursor_x = canvas_x;
                                app.cursor_y = canvas_y;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &App) -> Option<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main area
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    // Main area with sidebar and canvas
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // Sidebar
            Constraint::Min(0),     // Canvas
        ])
        .split(layout[0]);

    render_sidebar(frame, main_layout[0], app);
    render_canvas(frame, main_layout[1], app);

    // Status bar
    render_status_bar(frame, layout[1], app);

    // Return canvas area for mouse coordinate conversion
    Some(main_layout[1])
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    // Split sidebar into 3 sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // Tools section
            Constraint::Length(9),  // Elements section
            Constraint::Min(0),     // Properties section
        ])
        .split(area);

    // [1] Tools section
    let tools_title = if app.active_panel == Panel::Tools {
        "[1] Tools"
    } else {
        "[1] Tools"
    };
    let tools_style = if app.active_panel == Panel::Tools {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let tools = vec![
        Line::from(""),
        Line::from("  l - Line"),
        Line::from("  b - Box"),
        Line::from("  a - Arrow"),
        Line::from("  t - Text"),
        Line::from("  i - Insert"),
    ];

    let tools_block = Paragraph::new(tools)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(tools_title)
            .border_style(tools_style));

    frame.render_widget(tools_block, sections[0]);

    // [2] Elements section
    let elements_title = if app.active_panel == Panel::Elements {
        "[2] Elements"
    } else {
        "[2] Elements"
    };
    let elements_style = if app.active_panel == Panel::Elements {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let elements = vec![
        Line::from(""),
        Line::from("  (empty)"),
    ];

    let elements_block = Paragraph::new(elements)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(elements_title)
            .border_style(elements_style));

    frame.render_widget(elements_block, sections[1]);

    // [3] Properties section
    let props_title = if app.active_panel == Panel::Properties {
        "[3] Properties"
    } else {
        "[3] Properties"
    };
    let props_style = if app.active_panel == Panel::Properties {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let props = vec![
        Line::from(""),
        Line::from("  (no selection)"),
    ];

    let props_block = Paragraph::new(props)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(props_title)
            .border_style(props_style));

    frame.render_widget(props_block, sections[2]);
}

fn render_canvas(frame: &mut Frame, area: Rect, app: &App) {
    // Create a simple canvas representation
    let mut lines = vec![];

    for y in 0..area.height.saturating_sub(2) {
        let mut line_chars = vec![];
        for x in 0..area.width.saturating_sub(2) {
            if x == app.cursor_x && y == app.cursor_y {
                line_chars.push(Span::styled("█", Style::default().fg(Color::Yellow)));
            } else {
                line_chars.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(line_chars));
    }

    let canvas_title = "[0] Canvas";
    let canvas_style = if app.active_panel == Panel::Canvas {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let canvas = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(canvas_title)
            .border_style(canvas_style));

    frame.render_widget(canvas, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let status = Paragraph::new(Line::from(vec![
        Span::raw(" Cursor: ("),
        Span::raw(app.cursor_x.to_string()),
        Span::raw(", "),
        Span::raw(app.cursor_y.to_string()),
        Span::raw(") | Mode: "),
        Span::styled("Normal", Style::default().fg(Color::Green)),
        Span::raw(" | 0:Canvas 1:Tools 2:Elements 3:Props | q:Quit"),
    ]))
    .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    frame.render_widget(status, area);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Panel {
    Canvas = 0,
    Tools = 1,
    Elements = 2,
    Properties = 3,
}

struct App {
    cursor_x: u16,
    cursor_y: u16,
    canvas_area: Option<Rect>,
    active_panel: Panel,
}

impl App {
    fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            canvas_area: None,
            active_panel: Panel::Canvas,
        }
    }
}
