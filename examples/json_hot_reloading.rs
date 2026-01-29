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

    fn draw(t: &TaffyNode) {
        draw_rectangle_lines(
            t.layout.location.x + t.absolute_position.x,
            t.layout.location.y + t.absolute_position.y,
            t.layout.size.width,
            t.layout.size.height,
            2.0,
            BLACK,
        );
        for (_, rect) in t.children.iter() {
            draw(rect);
        }
    }
    let mut rects = None;
    loop {
        clear_background(WHITE);

        if let Some(contents) = reloader.update() {
            let layout_node = NodeJson::create_node(&contents);
            let n = Node::screen_root(layout_node);
            rects = TaffyNode::from_layout_node(n);
        }

        if let Some(ref rects) = rects {
            draw(rects);
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
