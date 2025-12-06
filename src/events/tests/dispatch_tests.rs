use super::fixtures::*;
use crate::app::App;
use crate::events::{EventHandler, handle_key_event, handle_mouse_event};

#[test]
fn test_event_chain_all_ignore() {
    let mut app = App::new();
    let handlers: Vec<&dyn EventHandler> = vec![&IgnoreHandler, &IgnoreHandler];

    let result = handle_key_event(&mut app, key_event('a'), &handlers).unwrap();

    assert_eq!(result, false);
}

#[test]
fn test_event_chain_first_consumes() {
    let mut app = App::new();
    let first = ConsumeHandler::new();
    let second = ConsumeHandler::new();
    let handlers: Vec<&dyn EventHandler> = vec![&first, &second];

    let result = handle_key_event(&mut app, key_event('a'), &handlers).unwrap();

    assert_eq!(result, false);
    assert!(first.was_called(), "First handler should be called");
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (propagation stopped)"
    );
}

#[test]
fn test_event_chain_second_consumes() {
    let mut app = App::new();
    let second = ConsumeHandler::new();
    let handlers: Vec<&dyn EventHandler> = vec![&IgnoreHandler, &second];

    let result = handle_key_event(&mut app, key_event('a'), &handlers).unwrap();

    assert_eq!(result, false);
    assert!(second.was_called(), "Second handler should be called");
}

#[test]
fn test_event_chain_quit_action() {
    let mut app = App::new();
    let handlers: Vec<&dyn EventHandler> = vec![&IgnoreHandler, &QuitHandler];

    let result = handle_key_event(&mut app, key_event('q'), &handlers).unwrap();

    assert_eq!(result, true);
}

#[test]
fn test_event_chain_quit_action_stops_propagation() {
    let mut app = App::new();
    let second = ConsumeHandler::new();
    let handlers: Vec<&dyn EventHandler> = vec![&QuitHandler, &second];

    let result = handle_key_event(&mut app, key_event('q'), &handlers).unwrap();

    assert_eq!(result, true);
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (quit stopped propagation)"
    );
}

#[test]
fn test_mouse_event_chain_consumes() {
    let mut app = App::new();
    let first = ConsumeHandler::new();
    let second = ConsumeHandler::new();
    let handlers: Vec<&dyn EventHandler> = vec![&first, &second];

    let result = handle_mouse_event(&mut app, mouse_down(), &handlers).unwrap();

    assert_eq!(result, false);
    assert!(first.was_called(), "First handler should be called");
    assert!(
        !second.was_called(),
        "Second handler should NOT be called (propagation stopped)"
    );
}

#[test]
fn test_mouse_event_chain_all_ignore() {
    let mut app = App::new();
    let handlers: Vec<&dyn EventHandler> = vec![&IgnoreHandler, &IgnoreHandler];

    let result = handle_mouse_event(&mut app, mouse_down(), &handlers).unwrap();

    assert_eq!(result, false);
}
