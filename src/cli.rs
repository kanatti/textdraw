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
