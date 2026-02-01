use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

pub fn draw(id: &String, taffy_node: &TaffyNode<DrawTag>, tag: Option<&DrawTag>) {
    draw_rectangle_lines(
        taffy_node.layout.location.x + taffy_node.absolute_position.x,
        taffy_node.layout.location.y + taffy_node.absolute_position.y,
        taffy_node.layout.size.width,
        taffy_node.layout.size.height,
        2.0,
        BLACK,
    );

    let ref current_tag = taffy_node.tag.as_ref().or(tag);
    if let Some(ref tag) = current_tag {
        let text = match tag.info {
            DrawTagText::None => "".to_string(),
            DrawTagText::Id => id.to_string(),
            DrawTagText::Text(ref text) => text.clone(),
        };

        let r = measure_text(&text, None, 16, 1.0);
        match tag.position.location(
            taffy_node.layout.size.width,
            taffy_node.layout.size.height,
            r.width,
            r.height,
        ) {
            Ok((x, y)) => {
                draw_text(
                    &text,
                    x,
                    y,
                    16.0,
                    helper::string_to_color(&tag.color).unwrap_or(BLACK),
                );
            }
            Err(e) => println!("Error {:?}", e),
        }
    }

    for (text, taffy_node) in taffy_node.children.iter() {
        draw(text, taffy_node, *current_tag);
    }
}
