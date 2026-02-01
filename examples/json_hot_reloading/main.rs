mod combox_component;
mod draw_macroquad_rects;
mod json_picker_component;
mod reloader;
mod update_result;
use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

use crate::reloader::BasicFileHotReloader;

fn conf() -> Conf {
    Conf {
        window_title: "json hot reloading".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let mut picker = json_picker_component::JsonPickerComponent::new();
    picker.start();
    let mut current_reloader: Option<BasicFileHotReloader> = None;

    let mut taffy_node = None;
    loop {
        clear_background(WHITE);

        if let Some(ref mut reloader) = current_reloader {
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
                draw_macroquad_rects::draw(&"".into(), taffy_node, taffy_node.tag.as_ref());
            }
        }
        picker.draw();
        match picker.update() {
            update_result::UpdateResult::Continue => {}
            update_result::UpdateResult::End(path) => {
                current_reloader = Some(BasicFileHotReloader::new(&path));
            }
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_readme() {
        let j =
            std::fs::read_to_string("examples/json_hot_reloading/assets/jsons/example_readme.json")
                .unwrap();
        let layout_node = json::json_to_node(&j).unwrap();
        let tree = TaffyNode::from_layout_node(layout_node).unwrap();

        assert_eq!(tree.layout.size.width, 800.0);
        assert_eq!(tree.layout.size.height, 600.0);
        assert_eq!(tree.children["header_node"].layout.size.width, 800.0);
        assert_eq!(tree.children["header_node"].layout.size.height, 100.0);
        assert_eq!(tree.children["body_node"].layout.size.width, 800.0);
        assert_eq!(tree.children["body_node"].layout.size.height, 500.0);
    }
}
