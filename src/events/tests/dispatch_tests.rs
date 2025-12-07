use super::fixtures::*;
use crate::AppState;
use crate::events::{EventHandler, handle_event};
use crossterm::event::Event;

#[test]
fn test_event_chain_all_ignore() {
    let mut state = AppState::new();
    let mut handler1 = IgnoreHandler;
    let mut handler2 = IgnoreHandler;
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut handler1, &mut handler2];

    let result = handle_event(Event::Key(key_event('a')), &mut handlers, &mut state).unwrap();

    assert_eq!(result, false);
}

#[test]
fn test_event_chain_first_consumes() {
    let mut state = AppState::new();
    let mut first = ConsumeHandler::new();
    let mut second = ConsumeHandler::new();
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut first, &mut second];

    let result = handle_event(Event::Key(key_event('a')), &mut handlers, &mut state).unwrap();

    assert_eq!(result, false);
    assert!(first.was_called(), "First handler should be called");
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (propagation stopped)"
    );
}

#[test]
fn test_event_chain_second_consumes() {
    let mut state = AppState::new();
    let mut handler1 = IgnoreHandler;
    let mut second = ConsumeHandler::new();
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut handler1, &mut second];

    let result = handle_event(Event::Key(key_event('a')), &mut handlers, &mut state).unwrap();

    assert_eq!(result, false);
    assert!(second.was_called(), "Second handler should be called");
}

#[test]
fn test_event_chain_quit_action() {
    let mut state = AppState::new();
    let mut handler1 = IgnoreHandler;
    let mut handler2 = QuitHandler;
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut handler1, &mut handler2];

    let result = handle_event(Event::Key(key_event('q')), &mut handlers, &mut state).unwrap();

    assert_eq!(result, true);
}

#[test]
fn test_event_chain_quit_action_stops_propagation() {
    let mut state = AppState::new();
    let mut handler1 = QuitHandler;
    let mut second = ConsumeHandler::new();
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut handler1, &mut second];

    let result = handle_event(Event::Key(key_event('q')), &mut handlers, &mut state).unwrap();

    assert_eq!(result, true);
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (quit stopped propagation)"
    );
}

#[test]
fn test_mouse_event_chain_consumes() {
    let mut state = AppState::new();
    let mut first = ConsumeHandler::new();
    let mut second = ConsumeHandler::new();
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut first, &mut second];

    let result = handle_event(Event::Mouse(mouse_down()), &mut handlers, &mut state).unwrap();

    assert_eq!(result, false);
    assert!(first.was_called(), "First handler should be called");
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (propagation stopped)"
    );
}

#[test]
fn test_mouse_event_chain_all_ignore() {
    let mut state = AppState::new();
    let mut handler1 = IgnoreHandler;
    let mut handler2 = IgnoreHandler;
    let mut handlers: Vec<&mut dyn EventHandler> = vec![&mut handler1, &mut handler2];

    let result = handle_event(Event::Mouse(mouse_down()), &mut handlers, &mut state).unwrap();

    assert_eq!(result, false);
}
