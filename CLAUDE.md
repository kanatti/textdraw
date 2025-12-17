# TextDraw - Development Guide

## Critical Rules

**NEVER use `cargo run`** - this is an interactive TUI that blocks the terminal
- Use `cargo check` to verify compilation
- Use `cargo test` for testing

**Always import types before using them:**
```rust
// ✅ Correct
use ratatui::style::Color;
let color = Color::Red;

// ❌ Wrong - don't use ratatui::style::Color::Red directly without importing
```

**Import specific items, not wildcards:**
- ✅ `use crate::ui::{PADDING_LEFT, CURSOR_BLOCK};`
- ❌ `use crate::ui::*;`

## Design System

**Use constants from `ui/styles.rs` - don't hardcode UI values:**
- Colors: `COLOR_PRIMARY`, `COLOR_ERROR`, `COLOR_SUCCESS`, `COLOR_HINT`
- Spacing: `PADDING_LEFT`, `SEPARATOR`
- Symbols: `CURSOR_BLOCK`

**Reusable patterns go in `ui/widgets.rs`:**
- `panel_block()`, `separator()`, `label_value_line()`, `ModalArea`
- Component-specific helpers stay local

## Code Patterns

**Prefer simple Ratatui syntax:**
- ✅ `Paragraph::new(format!("Value: {}", x))`
- ❌ `Line::from(vec![Span::raw("Value: "), Span::raw(x.to_string())])`
- Only use `Line::from(vec![Span...])` when mixing styles

**Extract duplication:**
- See it twice? Extract to helper/constant
- Modal logic → utilities
- Repeated UI → `ui/widgets.rs`
- Values → `ui/styles.rs`
