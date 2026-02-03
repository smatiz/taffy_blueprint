use macroquad::prelude::*;

pub enum ClickableResult {
    Hover,
    Clicked,
    None,
}

pub fn search(rect: &Rect) -> ClickableResult {
    if rect.contains(mouse_position().into()) {
        if is_mouse_button_pressed(MouseButton::Left) {
            ClickableResult::Clicked
        } else {
            ClickableResult::Hover
        }
    } else {
        ClickableResult::None
    }
}
