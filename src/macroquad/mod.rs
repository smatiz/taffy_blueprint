mod taffy_rect_node;

use crate::prelude::*;
use taffy::prelude::*;
pub use taffy_rect_node::*;

impl LayoutNode {
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
        Self::root(
            macroquad::window::screen_width(),
            macroquad::window::screen_height(),
            wrapped,
        )
    }
    pub fn grow(grow: f32) -> Self {
        Self::LeafAnonym(Style {
            flex_grow: grow,
            ..Default::default()
        })
    }
    pub fn macroquad_rect(&self) -> Option<TaffyRectNode> {
        TaffyNode::from_layout_node(self.clone()).map(|n| TaffyRectNode::new(n))
    }
}
