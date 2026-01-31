use crate::prelude::*;
use taffy::prelude::*;

impl<T> Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub fn screen_root(wrapped: Self) -> Self {
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
    pub fn macroquad_rect(&self) -> Result<TaffyRectNode<T>, TaffyBlueprintError> {
        TaffyNode::from_layout_node(self.clone()).map(TaffyRectNode::new)
    }
}
