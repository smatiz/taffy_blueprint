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
// mod style_as_string {
//     use super::Style;
//     use serde::{self, Deserialize, Deserializer, Serializer};
//     pub fn serialize<S>(style: &Style, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = format!(
//             "{}:{}:{}",
//             style.size.width., style.size.height, style.display
//         );
//         serializer.serialize_str(&s)
//     }
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Style, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         let parts: Vec<&str> = s.split(':').collect();
//         Ok(Style {
//             width: parts[0].parse().unwrap(),
//             height: parts[1].parse().unwrap(),
//             flex: parts[2].parse().unwrap(),
//         })
//     }
// }
#[macroquad::main(conf)]
async fn main() {
    let node = LayoutJson::create_node(
        r#"
         {
            id:"",
            style: {
                size: {
                    width: "100.0%",
                    height: "100.0%",
                },
                justify_content: Center,
                align_items: Center,
            },
            children: [{
                 id:"root",
                 style: {
                    size:  {
                        width: 100.0,
                        height: 100.0,
                    },
                    justify_content: Center,
                },
                [ {
                    id:"leaf",
                     style: {
                        size: {
                            width: 50%",
                            height: "*",
                        },
                    },
                )],
            ),
        )
        "#,
    );
    println!("{:?}", node);

    return;

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

    // let mut fx = f.debug_struct("Style");
    let default_style = LayoutNode::default();
    let diff = Diff::new(&default_style, &node);
    // Self::_print(&diff, 0, &mut fx);
    diff.print();
    // fx.finish()

    println!("--------------------------------------------");
    println!("{}", serde_json::to_string(&default_style).unwrap());
    println!("--------------------------------------------");
    println!("{}", serde_json::to_string(&node).unwrap());
    println!("--------------------------------------------");
    //     let node = serde_json::from_str::<LayoutNode>(
    //         r#"
    // LayoutNode {
    //         id:"",
    //         style: {
    //             size: Size {
    //                 width: percent(1.0),
    //                 height: percent(1.0),
    //             },
    //             justify_content: Some(JustifyContent::Center),
    //             align_items: Some(AlignItems::Center),
    //             ..Default::default()
    //         },
    //         children: [{
    //              id:"root".to_string(),
    //              style: {
    //                 size:  {
    //                     width: length(100.0),
    //                     height: length(100.0),
    //                 },
    //                 justify_content: Some(JustifyContent::Center),
    //             },
    //             [ {
    //                 id:"leaf",
    //                  style: {
    //                     size: {
    //                         width: percent(0.5),
    //                         height: auto(),
    //                     },
    //                 },
    //             )],
    //         ),
    //     )
    //     "#,
    //     );
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
