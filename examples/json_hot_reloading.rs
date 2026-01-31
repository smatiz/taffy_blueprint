use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Taffy Windows".to_string(),
        // window_width: 1920,
        // window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let mut reloader = BasicFileHotReloader::new("example.json");

    fn draw(taffy_node: &TaffyNode) {
        draw_rectangle_lines(
            taffy_node.layout.location.x + taffy_node.absolute_position.x,
            taffy_node.layout.location.y + taffy_node.absolute_position.y,
            taffy_node.layout.size.width,
            taffy_node.layout.size.height,
            2.0,
            BLACK,
        );
        for (_, taffy_node) in taffy_node.children.iter() {
            draw(taffy_node);
        }
    }
    let mut taffy_node = None;
    loop {
        clear_background(WHITE);

        if let Some(contents) = reloader.update() {
            match json_to_node(&contents) {
                Ok(layout_node) => {
                    let n = Node::screen_root(layout_node);
                    taffy_node = TaffyNode::from_layout_node(n);
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        if let Some(ref taffy_node) = taffy_node {
            draw(taffy_node);
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use taffy::prelude::*;

    use super::*;
    #[test]
    fn test_() {
        let lj = NodeJson::create_node(
            r#"
                {
                    "id": "a",
                    "style": {
                        "size": "123.4 234.5"
                    },
                    "children":[]
                }
        "#,
        );
        let l = Node::Layout(
            "a".into(),
            Style {
                size: Size {
                    width: length(123.4),
                    height: length(234.5),
                },
                ..Default::default()
            },
            vec![],
        );
        assert_eq!(lj, l);
    }
}
