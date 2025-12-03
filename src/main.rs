mod app;
mod canvas;
mod drawing;
mod element;
mod events;
mod tools;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
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
            ui::render(frame, &mut app);
        })?;

        // wait for next event
        let event = crossterm::event::read()?;
        let should_quit = events::handle_event(&mut app, event)?;

        if should_quit {
            break;
        }
    }

    Ok(())
}
