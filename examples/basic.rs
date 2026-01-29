use macroquad::prelude::*;
use taffy::prelude::*;
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
    let node = Node::Anonym(
        Style {
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..Default::default()
        },
        vec![Node::Layout(
            "root".to_string(),
            Style {
                size: Size {
                    width: length(100.0),
                    height: length(100.0),
                },
                justify_content: Some(JustifyContent::Center),
                ..Default::default()
            },
            vec![Node::Leaf(
                "leaf".to_string(),
                Style {
                    size: Size {
                        width: percent(0.5),
                        height: auto(),
                    },
                    ..Default::default()
                },
            )],
        )],
    );
    let n = Node::screen_root(node);
    let rects = TaffyNode::from_layout_node(n);

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

    loop {
        clear_background(WHITE);
        if let Some(ref rects) = rects {
            draw(rects);
        }
        next_frame().await;
    }
}
