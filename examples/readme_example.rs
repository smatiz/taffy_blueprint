use taffy::prelude::*;
use taffy_blueprint::prelude::*;

fn main() {
    let root = LayoutNode::Node(
        "root".to_string(),
        Style {
            flex_direction: FlexDirection::Column,
            size: Size {
                width: length(800.0),
                height: length(600.0),
            },
            ..Default::default()
        },
        vec![
            LayoutNode::Leaf(
                "header_node".to_string(),
                Style {
                    size: Size {
                        width: length(800.0),
                        height: length(100.0),
                    },
                    ..Default::default()
                },
            ),
            LayoutNode::Leaf(
                "body_node".to_string(),
                Style {
                    size: Size {
                        width: length(800.0),
                        height: auto(),
                    },
                    flex_grow: 1.0,
                    ..Default::default()
                },
            ),
        ],
    );

    let tree = TaffyNode::from_layout_node(root).unwrap();
    assert_eq!(tree.layout.size.width, 800.0);
    assert_eq!(tree.layout.size.height, 600.0);
    assert_eq!(tree.children["header_node"].layout.size.width, 800.0);
    assert_eq!(tree.children["header_node"].layout.size.height, 100.0);
    assert_eq!(tree.children["body_node"].layout.size.width, 800.0);
    assert_eq!(tree.children["body_node"].layout.size.height, 500.0);
}
