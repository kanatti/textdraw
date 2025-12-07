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

The main loop renders UI from application state, waits for user input, then dispatches events to modify state. The next loop frame renders UI based on the modified state.

## Event Handlers

The `EventHandler` trait provides methods for handling keyboard and mouse events (`handle_key_event`, `handle_mouse_down`, etc.). Handlers return `EventResult` to control event propagation—events stop at the first handler that consumes them.

## Components

The `Component` trait defines UI elements that can render themselves (`draw` method). All components must also implement `EventHandler` (via trait bound `Component: EventHandler`), ensuring every UI component can participate in event handling.

## Event Dispatching

At any time, one component is active in the application (Canvas, Tools Panel, Properties Panel, Help Modal, etc.). Events are dispatched to components in priority order—each component can consume or ignore the event. Typically, inactive components ignore events while the active component consumes them. If all components ignore an event, it falls back to the `GlobalEventHandler` for application-wide actions like quit.
