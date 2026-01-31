use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

pub fn string_to_color(s: &str) -> Option<Color> {
    let s = s.trim().to_lowercase();

    // 1. Nomi predefiniti
    match s.as_str() {
        "white" => return Some(WHITE),
        "black" => return Some(BLACK),
        "red" => return Some(RED),
        "green" => return Some(GREEN),
        "blue" => return Some(BLUE),
        "yellow" => return Some(YELLOW),
        "orange" => return Some(ORANGE),
        "pink" => return Some(PINK),
        "purple" => return Some(PURPLE),
        "gray" | "grey" => return Some(GRAY),
        _ => {}
    }

    // 2. Hex: #RRGGBB o #RRGGBBAA (con o senza #)
    let hex = if s.starts_with('#') { &s[1..] } else { &s };

    fn parse_hex_pair(pair: &str) -> Option<u8> {
        u8::from_str_radix(pair, 16).ok()
    }

    match hex.len() {
        6 => {
            let r = parse_hex_pair(&hex[0..2])?;
            let g = parse_hex_pair(&hex[2..4])?;
            let b = parse_hex_pair(&hex[4..6])?;
            Some(Color::from_rgba(r, g, b, 255))
        }
        8 => {
            let r = parse_hex_pair(&hex[0..2])?;
            let g = parse_hex_pair(&hex[2..4])?;
            let b = parse_hex_pair(&hex[4..6])?;
            let a = parse_hex_pair(&hex[6..8])?;
            Some(Color::from_rgba(r, g, b, a))
        }
        _ => None,
    }
}

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

    fn draw(id: &String, taffy_node: &TaffyNode<DrawTag>) {
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
                DrawTagText::None => "".to_string(),
                DrawTagText::Id => id.to_string(),
                DrawTagText::Text(ref text) => text.clone(),
            };

            let r = measure_text(&text, None, 16, 1.0);
            let node = tag.position.node(
                taffy_node.layout.size.width,
                taffy_node.layout.size.height,
                r.width,
                r.height,
            );

            match TaffyNode::from_layout_node(node) {
                Ok(taffy_node) => {
                    let ref d = taffy_node.children["debug"];
                    let x = d.layout.location.x;
                    let y = d.layout.location.y;

                    draw_text(
                        &text,
                        x,
                        y,
                        16.0,
                        string_to_color(&tag.color).unwrap_or(BLACK),
                    );
                }
                Err(e) => println!("Error {:?}", e),
            }
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
            match json::json_to_node(&contents) {
                Ok(layout_node) => {
                    let n = Node::screen_root(layout_node);
                    taffy_node = TaffyNode::from_layout_node(n).ok();
                }
                Err(e) => println!("Error: {:?}", e),
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
