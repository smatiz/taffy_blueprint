use crate::prelude::*;
use taffy::prelude::*;

impl LayoutNode {
    pub fn screen_root(wrapped: LayoutNode) -> LayoutNode {
        LayoutNode::Node(
            "root".to_string(),
            Style {
                size: Size {
                    width: length(macroquad::window::screen_width()),
                    height: length(macroquad::window::screen_height()),
                },
                ..Default::default()
            },
            vec![wrapped],
        )
    }
    pub fn macroquad_rect(&self) -> Option<TaffyRectNode> {
        TaffyNode::from_layout_node(self.clone()).map(|n| TaffyRectNode::new(n))
    }
}
