use macroquad::prelude::*;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

pub fn wrapped_into_screen_root_node(node: Node<()>) -> TaffyRectNode<()> {
    TaffyRectNode::new(
        TaffyNode::from_layout_node(Node::Layout(
            "root".into(),
            Style {
                size: Size {
                    width: length(screen_width()),
                    height: length(screen_height()),
                },
                ..Default::default()
            },
            vec![node],
        ))
        .unwrap(),
    )
}
