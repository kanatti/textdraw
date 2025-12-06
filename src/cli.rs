use anyhow::Result;
use crate::canvas::Canvas;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "textdraw")]
#[command(about = "An interactive terminal ASCII diagram editor", long_about = None)]
pub struct Cli {
    /// File to open (or render with --render flag)
    #[arg(value_name = "FILE")]
    pub file: Option<String>,

    /// Render the file to the terminal without entering TUI mode
    #[arg(short, long)]
    pub render: bool,
}

/// Render a diagram file to stdout without entering TUI mode
pub fn render_file(file_path: &str) -> Result<()> {
    // Load the file
    let mut canvas = Canvas::default();
    canvas.load_from_file(file_path)?;

    // Check if canvas has any elements
    if canvas.is_empty() {
        println!("(empty diagram)");
        return Ok(());
    }

    // Get bounding box of all elements
    let (min_x, min_y, max_x, max_y) = canvas.bounds();

    // Build entire output string
    let mut output = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let ch = canvas.get(x, y).unwrap_or(' ');
            output.push(ch);
        }
        output.push('\n');
    }

    // Print in one shot
    print!("{}", output);

    Ok(())
}
