mod reloader;
use macroquad::prelude::*;
use notify::{RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
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
    let mut reloader = reloader::FileReloader::new("config.json");

    fn draw(t: &TaffyRectNode) {
        draw_rectangle_lines(t.rect().x, t.rect().y, t.rect().w, t.rect().h, 2.0, BLACK);
        for (_, rect) in t.get_all_children().iter() {
            draw(rect);
        }
    }
    let mut rects = None;
    loop {
        clear_background(WHITE);

        if let Some(contents) = reloader.update() {
            println!("contents {}", contents);
            let node = LayoutJson::create_node(&contents);
            let mut taffy = TaffyTree::<()>::new();
            rects = Some(LayoutNode::screen_root(node).macroquad_rect(&mut taffy));
        }

        if let Some(ref rects) = rects {
            draw(rects);
        }
        next_frame().await;
    }
}
