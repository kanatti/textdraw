use crate::state::CanvasState;
use std::path::Path;

pub struct FileState {
    pub current_file: Option<String>,
    pub status_message: Option<String>,
}

impl FileState {
    pub fn new() -> Self {
        Self {
            current_file: None,
            status_message: None,
        }
    }

    // Status message management

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
    }

    pub fn clear_status_message(&mut self) {
        self.status_message = None;
    }

    // File I/O operations

    /// Save the diagram to a file
    pub fn save_to_file(
        &mut self,
        canvas: &CanvasState,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        canvas.save_to_file(&path)?;
        self.current_file = Some(path.as_ref().display().to_string());
        self.status_message = Some(format!("Saved to {}", path.as_ref().display()));
        Ok(())
    }

    /// Load a diagram from a file
    pub fn load_from_file(
        &mut self,
        canvas: &mut CanvasState,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        self.load_from_file_with_message(canvas, &path, true)
    }

    /// Load a diagram from a file, optionally showing a status message
    fn load_from_file_with_message(
        &mut self,
        canvas: &mut CanvasState,
        path: impl AsRef<Path>,
        show_message: bool,
    ) -> anyhow::Result<()> {
        canvas.load_from_file(&path)?;
        self.current_file = Some(path.as_ref().display().to_string());
        if show_message {
            self.status_message = Some(format!("Loaded from {}", path.as_ref().display()));
        }
        Ok(())
    }

    /// Load a diagram from a file silently (for initial load)
    pub fn load_from_file_silent(
        &mut self,
        canvas: &mut CanvasState,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        self.load_from_file_with_message(canvas, &path, false)
    }
}
