use crate::prelude::*;
use taffy::prelude::*;

impl Node {
    pub fn screen_root(wrapped: Node) -> Node {
        Self::Layout(
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
        TaffyNode::from_layout_node(self.clone()).map(TaffyRectNode::new)
    }
}
