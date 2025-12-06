use assert_cmd::cargo;
use assert_cmd::prelude::*;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::path::Path;
use std::process::Command;

/// Temporary diagram file that cleans up after itself
struct TempDiagramFile {
    _temp: TempDir,
    file: ChildPath,
}

impl TempDiagramFile {
    fn new(name: &str, content: &str) -> Self {
        let temp = TempDir::new().unwrap();
        let file = temp.child(name);
        file.write_str(content).unwrap();
        Self { _temp: temp, file }
    }

    fn path(&self) -> &Path {
        self.file.path()
    }
}

/// Helper to create a Command for the textdraw binary
fn textdraw_cmd() -> Command {
    Command::new(cargo::cargo_bin!("textdraw"))
}

/// Test that --help and -h flags display help information
#[test]
fn test_help() {
    let expected = "\
An interactive terminal ASCII diagram editor

Usage: textdraw [OPTIONS] [FILE]

Arguments:
  [FILE]  File to open (or render with --render flag)

Options:
  -r, --render  Render the file to the terminal without entering TUI mode
  -h, --help    Print help
";

    // Test --help
    textdraw_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(expected);

    // Test -h
    textdraw_cmd()
        .arg("-h")
        .assert()
        .success()
        .stdout(expected);
}

/// Test rendering a diagram file to stdout with --render and -r flags
#[test]
fn test_render() {
    let diagram_file = TempDiagramFile::new("test.textdraw", r#"{
  "version": "0.1.0",
  "elements": [
    {
      "Rectangle": {
        "id": 0,
        "name": "Rectangle 1",
        "top_left": [0, 0],
        "bottom_right": [5, 3]
      }
    }
  ],
  "next_id": 1
}"#);

    let expected = "\
┌────┐
│    │
│    │
└────┘
";

    // Test --render
    textdraw_cmd()
        .arg("--render")
        .arg(diagram_file.path())
        .assert()
        .success()
        .stdout(expected);

    // Test -r
    textdraw_cmd()
        .arg("-r")
        .arg(diagram_file.path())
        .assert()
        .success()
        .stdout(expected);
}

/// Test rendering an empty diagram
#[test]
fn test_render_empty_file() {
    let diagram_file = TempDiagramFile::new("empty.textdraw", r#"{
  "version": "0.1.0",
  "elements": [],
  "next_id": 0
}"#);

    textdraw_cmd()
        .arg("--render")
        .arg(diagram_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("(empty diagram)"));
}

/// Test that --render without a file argument fails
#[test]
fn test_render_missing_file() {
    textdraw_cmd()
        .arg("--render")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--render requires a file argument"));
}

/// Test rendering a non-existent file
#[test]
fn test_render_nonexistent_file() {
    let temp = TempDir::new().unwrap();
    let nonexistent = temp.path().join("nonexistent.textdraw");

    textdraw_cmd()
        .arg("--render")
        .arg(&nonexistent)
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
}

/// Test rendering an invalid JSON file
#[test]
fn test_render_invalid_json() {
    let diagram_file = TempDiagramFile::new("invalid.textdraw", "not valid json");

    textdraw_cmd()
        .arg("--render")
        .arg(diagram_file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse diagram file"));
}
