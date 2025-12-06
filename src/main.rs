mod app;
mod canvas;
mod cli;
mod components;
mod drawing;
mod element;
mod events;
mod input;
mod tools;
mod types;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    // Handle render mode
    if cli.render {
        if let Some(file_path) = cli.file {
            return cli::render_file(&file_path);
        } else {
            eprintln!("Error: --render requires a file argument");
            std::process::exit(1);
        }
    }

    // Normal TUI mode
    // Enable mouse capture
    execute!(std::io::stdout(), EnableMouseCapture)?;

    let terminal = ratatui::init();
    let result = run(terminal, cli.file);
    ratatui::restore();

    // Disable mouse capture
    execute!(std::io::stdout(), DisableMouseCapture)?;

    result
}

fn run(mut terminal: DefaultTerminal, file: Option<String>) -> Result<()> {
    let mut app = App::new();

    // Load file if provided (silently, no status message)
    if let Some(file_path) = file {
        if let Err(e) = app.load_from_file_silent(&file_path) {
            eprintln!("Error loading file: {}", e);
            std::process::exit(1);
        }
    }

    let handlers = events::default_handlers();

    // Main render and event loop.
    //
    // ratatui follows immediate mode rendering where the entire UI is redrawn every frame.
    // This loop combines rendering and event handling in a single loop instead of having
    // separate render and event loops.
    //
    // Flow:
    // 1. Render the current app state
    // 2. Block waiting for an event (keyboard, mouse, etc.)
    // 3. Handle the event and mutate app state
    // 4. Loop back to re-render with updated state
    //
    // Important: Since we use blocking event reads (event::read()), the UI is only redrawn
    // after an event occurs. This means if app state is mutated outside of event handling,
    // the UI will be out of sync until the next event. For our case, all state changes
    // happen through events, so this simple blocking approach works well.
    //
    // Alternative: We could use polling (event::poll()) with separate render/event loops
    // to support async state updates, but that adds complexity we don't currently need.
    loop {
        // Render phase: Draw UI based on current app state (read-only)
        terminal.draw(|frame| {
            app.layout = ui::calculate_layout(frame);
            ui::render(frame, &app);
        })?;

        // Event phase: Block until next event, then handle it (mutates app state)
        let event = crossterm::event::read()?;
        let should_quit = events::handle_event(&mut app, event, &handlers)?;

        if should_quit {
            break;
        }
    }

    Ok(())
}
