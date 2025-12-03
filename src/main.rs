mod app;
mod canvas;
mod drawing;
mod events;
mod tools;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use ratatui::DefaultTerminal;

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
            let areas = ui::render(frame, &app);
            app.canvas_area = Some(areas.canvas);
            app.tools_area = Some(areas.tools);
            app.elements_area = Some(areas.elements);
            app.properties_area = Some(areas.properties);
        })?;

        let event = event::read()?;
        let should_quit = events::handle_event(&mut app, event)?;

        if should_quit {
            break;
        }
    }

    Ok(())
}

