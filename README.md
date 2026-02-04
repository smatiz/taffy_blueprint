
[Taffy](https://github.com/DioxusLabs/taffy) is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Taffy Blueprint is built on top of it.

The goal is to offer a layer for people who prefer **declarative** workflows in Rust or want rapid iteration during **prototyping**  (or even just learn Taffy API **hands-on**) with Json.


## Usage

```rust
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

fn main() {
    let root = Node::<()>::Layout(
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
            Node::Leaf(
                "header_node".to_string(),
                Style {
                    size: Size {
                        width: length(800.0),
                        height: length(100.0),
                    },
                    ..Default::default()
                },
            ),
            Node::Leaf(
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

```
or the same in Json

```json
{
  "id": "root",
  "style": {
    "flex_direction": "Column",
    "size": "800.0 600.0"
  },
  "children": [
    {
      "id": "header_node",
      "style": {
        "size": "800.0 100.0"
      },
      "children": []
    },
    {
      "id": "body_node",
      "style": {
        "size": "800.0 *",
        "flex_grow": 1.0
      },
      "children": []
    }
  ]
}

```

Some examples are available using Macroquad for rendering:

```shell
cargo run --example json_hot_reloading --features use_json --features use_macroquad
```
