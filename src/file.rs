use crate::canvas::Canvas;
use crate::element::Element;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Serializable format for saving/loading diagrams
#[derive(Serialize, Deserialize)]
pub struct DiagramFile {
    pub version: String,
    pub elements: Vec<Element>,
    pub next_id: usize,
}

impl DiagramFile {
    pub fn new(elements: Vec<Element>, next_id: usize) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            elements,
            next_id,
        }
    }

    /// Save diagram to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize diagram")?;

        fs::write(path.as_ref(), json).context(format!(
            "Failed to write to file: {}",
            path.as_ref().display()
        ))?;

        Ok(())
    }

    /// Load diagram from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let json = fs::read_to_string(path.as_ref())
            .context(format!("Failed to read file: {}", path.as_ref().display()))?;

        let diagram: DiagramFile =
            serde_json::from_str(&json).context("Failed to parse diagram file")?;

        Ok(diagram)
    }
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
