
[Taffy](https://github.com/DioxusLabs/taffy) is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Taffy Blueprint is built on top of it.
It provides a declarative LayoutNode for building your UI, supports JSON hot‑reloading for rapid prototyping, and includes a practical Macroquad integration example.

## Usage

```rust
  use taffy::prelude::*;
  use taffy_blueprint::prelude::*;

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

```
