use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

pub fn draw(id: &String, taffy_node: &TaffyNode<DrawTag>, tag: Option<&DrawTag>) {
    // println!("taffy_node.tag {:?}", taffy_node.tag);
    // println!("tag {:?}", tag);
    let ref current_tag = taffy_node.tag.as_ref().or(tag);
    let color = if let Some(ref tag) = current_tag {
        let text = match tag.info {
            DrawTagText::None => None,
            DrawTagText::Id => Some(id.to_string()),
            DrawTagText::Text(ref text) => Some(text.clone()),
        };
        if let Some(text) = text {
            let color = helper::string_to_color(&tag.color).unwrap_or(BLACK);
            // println!("current_tag {:?}", tag.info);
            let r = measure_text(&text, None, 16, 1.0);
            match tag.position.location(
                taffy_node.layout.size.width,
                taffy_node.layout.size.height,
                r.width,
                r.height,
            ) {
                Ok(Some((x, y))) => {
                    draw_text(
                        &text,
                        x + taffy_node.layout.location.x + taffy_node.absolute_position.x,
                        y + taffy_node.layout.location.y
                            + taffy_node.absolute_position.y
                            + r.offset_y,
                        16.0,
                        color,
                    );
                }
                Err(e) => println!("Error {:?}", e),
                _ => {}
            }
            Some(color)
        } else {
            None
        }
    } else {
        Some(BLACK)
    };
    if let Some(color) = color {
        draw_rectangle_lines(
            taffy_node.layout.location.x + taffy_node.absolute_position.x,
            taffy_node.layout.location.y + taffy_node.absolute_position.y,
            taffy_node.layout.size.width,
            taffy_node.layout.size.height,
            2.0,
            color,
        );
    }
    for (text, taffy_node) in taffy_node.children.iter() {
        draw(text, taffy_node, *current_tag);
    }
}
