use crate::app::App;

/// Actions that can be triggered by command execution
#[derive(Debug, Clone, PartialEq)]
pub enum CommandAction {
    /// Save to a specific file
    Save(String),
    /// Save to current file (no path specified)
    SaveCurrent,
    /// Open a file
    Open(String),
    /// Show a message (for quit or unknown commands)
    Message(String),
    /// No action (empty command)
    None,
}

pub struct CommandState {
    pub buffer: String,
    pub active: bool,
}

impl CommandState {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            active: false,
        }
    }

    pub fn enter(&mut self) {
        self.active = true;
        self.buffer.clear();
    }

    pub fn enter_with(&mut self, command: &str) {
        self.active = true;
        self.buffer = command.to_string();
    }

    pub fn exit(&mut self) {
        self.active = false;
        self.buffer.clear();
    }

    /// Exit command mode without clearing - used after execution to keep status message
    pub fn finish(&mut self) {
        self.active = false;
        self.buffer.clear();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn add_char(&mut self, c: char) {
        if self.active {
            self.buffer.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.active {
            self.buffer.pop();
        }
    }

    /// Parse the command and return the action to execute
    pub fn parse(&self) -> CommandAction {
        if !self.active {
            return CommandAction::None;
        }

        let command = self.buffer.trim();

        // Parse command
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return CommandAction::None;
        }

        match parts[0] {
            "save" | "s" | "w" => {
                // :save filename or :w filename
                if parts.len() > 1 {
                    let filename = parts[1..].join(" ");
                    let full_path = if filename.ends_with(".textdraw") {
                        filename
                    } else {
                        format!("{}.textdraw", filename)
                    };
                    CommandAction::Save(full_path)
                } else {
                    CommandAction::SaveCurrent
                }
            }
            "open" | "o" | "e" => {
                // :open filename or :e filename
                if parts.len() > 1 {
                    let filename = parts[1..].join(" ");
                    let full_path = if filename.ends_with(".textdraw") {
                        filename
                    } else {
                        format!("{}.textdraw", filename)
                    };
                    CommandAction::Open(full_path)
                } else {
                    CommandAction::Message("No filename specified".to_string())
                }
            }
            "q" | "quit" => {
                // Handle quit - you might want to add a confirmation if unsaved
                // For now, just show a message
                CommandAction::Message("Use 'q' key to quit".to_string())
            }
            _ => CommandAction::Message(format!("Unknown command: {}", parts[0])),
        }
    }
}

/// Executes command actions on the app
pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(action: CommandAction, app: &mut App) {
        match action {
            CommandAction::Save(path) => {
                if let Err(e) = app.file.save_to_file(&app.canvas, &path) {
                    app.file.status_message = Some(format!("Error: {}", e));
                }
            }
            CommandAction::SaveCurrent => {
                if let Some(current) = app.file.current_file.clone() {
                    if let Err(e) = app.file.save_to_file(&app.canvas, &current) {
                        app.file.status_message = Some(format!("Error: {}", e));
                    }
                } else {
                    app.file.status_message = Some("No filename specified".to_string());
                }
            }
            CommandAction::Open(path) => {
                if let Err(e) = app.file.load_from_file(&mut app.canvas, &path) {
                    app.file.status_message = Some(format!("Error: {}", e));
                }
            }
            CommandAction::Message(msg) => {
                app.file.status_message = Some(msg);
            }
            CommandAction::None => {}
        }
    }
}
