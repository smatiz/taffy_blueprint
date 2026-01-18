mod reloader;
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
    let mut reloader = reloader::FileReloader::new("config.json");

    fn draw(t: &TaffyLayoutNode) {
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
            println!("contents {}", contents);
            let layout_node = LayoutJson::create_node(&contents);
            rects = TaffyLayoutNode::new(screen_root(layout_node));
        }

        if let Some(ref rects) = rects {
            draw(rects);
        }
        next_frame().await;
    }
}
