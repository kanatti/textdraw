use crate::element::Element;
use crate::state::CanvasState;

pub fn add_element(canvas: &mut CanvasState, element: Element) -> usize {
    canvas.add_element(element)
}

pub fn remove_element(canvas: &mut CanvasState, id: usize) -> Option<Element> {
    canvas.remove_element(id)
}

pub fn get_char_at(canvas: &CanvasState, x: i32, y: i32) -> Option<char> {
    canvas.get(x, y)
}

pub fn find_element_at(canvas: &CanvasState, x: i32, y: i32) -> Option<usize> {
    canvas.find_element_at(x, y)
}

pub fn find_elements_in_rect(
    canvas: &CanvasState,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> Vec<usize> {
    canvas.find_elements_in_rect(x1, y1, x2, y2)
}

pub fn find_elements_fully_inside_rect(
    canvas: &CanvasState,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> Vec<usize> {
    canvas.find_elements_fully_inside_rect(x1, y1, x2, y2)
}

pub fn get_element(canvas: &CanvasState, id: usize) -> Option<&Element> {
    canvas.get_element(id)
}

pub fn get_element_mut(canvas: &mut CanvasState, id: usize) -> Option<&mut Element> {
    canvas.get_element_mut(id)
}

pub fn elements(canvas: &CanvasState) -> &[Element] {
    canvas.elements()
}

pub fn bounds(canvas: &CanvasState) -> (i32, i32, i32, i32) {
    canvas.bounds()
}

pub fn is_empty(canvas: &CanvasState) -> bool {
    canvas.is_empty()
}

pub fn element_contains_point(element: &Element, x: i32, y: i32) -> bool {
    element.contains_point(x, y)
}

pub fn element_point_in_bounds(element: &Element, x: i32, y: i32) -> bool {
    element.point_in_bounds(x, y)
}

pub fn translate_element(element: &mut Element, dx: i32, dy: i32) {
    element.translate(dx, dy);
}
