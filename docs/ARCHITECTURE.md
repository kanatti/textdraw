# Architecture

## Overview

```
              ┌──────────────────┐
              │     AppState     │◄──────────────┐
              └──────┬───────────┘               │
                     │                           │
        ┌────────────┴────────────┐            mutates
      reads                     reads            │
        │                         │              │
        ▼                         ▼              │
   ┌────────────┐          ┌──────────────────┐  │
   │ ui::render │          │ events::dispatch │──┘
   └────────────┘          └──────────────────┘
```

The main loop renders UI from application state, waits for user input, then dispatches events to modify state. The next loop frame renders UI based on the modified state. See `main.rs` for more details on the rendering loop.

## State

`AppState` is the central state container and entry point for all state access. Some state is delegated to sub-modules (`CanvasState`, `ToolState`, `FileState`, etc.), but all mutating actions go through `AppState` methods. This makes `AppState` act like a store that holds both state and actions.

## Event Handlers

The `EventHandler` trait provides methods for handling keyboard and mouse events (`handle_key_event`, `handle_mouse_down`, etc.). Handlers return `EventResult` to control event propagation—events stop at the first handler that consumes them.

## Components

The `Component` trait defines UI elements that can render themselves (`draw` method). All components must also implement `EventHandler` (via trait bound `Component: EventHandler`), ensuring every UI component can participate in event handling.

Components can optionally be stateful to keep local state that doesn't belong in the global `AppState` (e.g., scroll position in `HelpModal`). Though, note that all components are created once during app startup, so their state persists between hiding and showing (e.g., modal scroll position is preserved).

## Event Dispatching

At any time, one component is active in the application (Canvas, Tools Panel, Properties Panel, Help Modal, etc.). Events are dispatched to components in priority order—each component can consume or ignore the event. Typically, inactive components ignore events while the active component consumes them. If all components ignore an event, it falls back to the `GlobalEventHandler` for application-wide actions like quit.
