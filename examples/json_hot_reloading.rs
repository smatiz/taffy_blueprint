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

    fn draw(id: &String, taffy_node: &TaffyNode<DebugLabel>) {
        draw_rectangle_lines(
            taffy_node.layout.location.x + taffy_node.absolute_position.x,
            taffy_node.layout.location.y + taffy_node.absolute_position.y,
            taffy_node.layout.size.width,
            taffy_node.layout.size.height,
            2.0,
            BLACK,
        );

        if let Some(ref tag) = taffy_node.tag {
            let text = match tag.info {
                DebugLabelText::None => "".to_string(),
                DebugLabelText::Id => id.to_string(),
                DebugLabelText::Text(ref text) => text.clone(),
            };

            let r = measure_text(&text, None, 16, 1.0);
            let node = tag.position.place(r.width, r.height);

            let taffy_node = TaffyNode::from_layout_node(node).unwrap();
            let ref d = taffy_node.children["debug"];
            let x = d.layout.location.x;
            let y = d.layout.location.y;

            draw_text(&text, x, y, 16.0, RED);
        }

        // println!("taffy_node.debug: {:?}", taffy_node.tag);
        for (text, taffy_node) in taffy_node.children.iter() {
            draw(text, taffy_node);
        }
    }
    let mut taffy_node = None;
    loop {
        clear_background(WHITE);

        if let Some(contents) = reloader.update() {
            match json_to_node(&contents) {
                Ok(layout_node) => {
                    let n = Node::screen_root(layout_node);
                    taffy_node = TaffyNode::from_layout_node(n).ok();
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        if let Some(ref taffy_node) = taffy_node {
            draw(&"".into(), taffy_node);
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
