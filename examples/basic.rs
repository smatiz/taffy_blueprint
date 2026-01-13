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
    let node = LayoutNode::single_anonym(
        Style {
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..Default::default()
        },
        LayoutNode::new(
            "root".to_string(),
            Style {
                size: Size {
                    width: length(100.0),
                    height: length(100.0),
                },
                justify_content: Some(JustifyContent::Center),
                ..Default::default()
            },
            vec![LayoutNode::leaf(
                "leaf".to_string(),
                Style {
                    size: Size {
                        width: percent(0.5),
                        height: auto(),
                    },
                    ..Default::default()
                },
            )],
        ),
    );
    let mut taffy = TaffyTree::<()>::new();
    let rects = LayoutNode::screen_root(node).macroquad_rect(&mut taffy);

    fn draw(t: &TaffyRectNode) {
        draw_rectangle_lines(t.rect().x, t.rect().y, t.rect().w, t.rect().h, 2.0, BLACK);
        for (_, rect) in t.get_all_children().iter() {
            draw(rect);
        }
    }

    loop {
        clear_background(WHITE);
        draw(&rects);
        next_frame().await;
    }
}

// fn main() -> Result<(), taffy::TaffyError> {
//     let mut taffy: TaffyTree<()> = TaffyTree::new();

//     let child = taffy.new_leaf(Style {
//         size: Size {
//             width: Dimension::from_percent(0.5),
//             height: Dimension::AUTO,
//         },
//         ..Default::default()
//     })?;

//     let node = taffy.new_with_children(
//         Style {
//             size: Size {
//                 width: Dimension::from_length(100.0),
//                 height: Dimension::from_length(100.0),
//             },
//             justify_content: Some(JustifyContent::Center),
//             ..Default::default()
//         },
//         &[child],
//     )?;

//     println!("Compute layout with 100x100 viewport:");
//     taffy.compute_layout(
//         node,
//         Size {
//             height: AvailableSpace::Definite(100.0),
//             width: AvailableSpace::Definite(100.0),
//         },
//     )?;
//     println!("node: {:#?}", taffy.layout(node)?);
//     println!("child: {:#?}", taffy.layout(child)?);

//     println!("Compute layout with undefined (infinite) viewport:");
//     taffy.compute_layout(node, Size::MAX_CONTENT)?;
//     println!("node: {:#?}", taffy.layout(node)?);
//     println!("child: {:#?}", taffy.layout(child)?);

//     Ok(())
// }
