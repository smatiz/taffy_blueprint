use crate::prelude::*;
use taffy::prelude::*;

fn root(width: f32, height: f32, wrapped: LayoutNode) -> LayoutNode {
    LayoutNode::Node(
        "root".to_string(),
        Style {
            size: Size {
                width: length(width),
                height: length(height),
            },
            ..Default::default()
        },
        vec![wrapped],
    )
}
pub fn screen_root(wrapped: LayoutNode) -> LayoutNode {
    root(
        macroquad::window::screen_width(),
        macroquad::window::screen_height(),
        wrapped,
    )
}
