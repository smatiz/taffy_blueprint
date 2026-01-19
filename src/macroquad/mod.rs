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

    pub fn macroquad_rect(&self) -> TaffyRectNode {
        let n = TaffyLayoutNode::new(self.clone());
        // TODO remove unwrap
        TaffyRectNode::new(n.unwrap())
        // let taffy_root = _to_taffy(taffy, n);
        // taffy
        //     .compute_layout(taffy_root.id, Size::MAX_CONTENT)
        //     .unwrap();
        // taffy_root.to_macroquad(taffy)
    }
}
