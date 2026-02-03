mod board;
mod card;
mod character;
pub mod h_clickable;
mod label;
mod taffy_child;
mod text_drawer;
mod types;
mod value_component;

use macroquad::prelude::*;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

use crate::{
    board::Board,
    character::{Character, Class},
    text_drawer::TextDrawer,
};

fn conf() -> Conf {
    Conf {
        window_title: "cards".to_string(),
        fullscreen: false,
        ..Default::default()
    }
}

// Mage,
// Warrior,
// Rogue,
#[macroquad::main(conf)]
async fn main() {
    let mut board = Board::new();
    let text_drawer = TextDrawer::new(16).await;
    board.start(vec![
        Character {
            class: Class::Mage,
            agility: 2,
            intelligence: 3,
            mana: 6,
            strength: 1,
        },
        Character {
            class: Class::Warrior,
            agility: 0,
            intelligence: 1,
            mana: 0,
            strength: 6,
        },
        Character {
            class: Class::Rogue,
            agility: 5,
            intelligence: 4,
            mana: 0,
            strength: 2,
        },
    ]);
    loop {
        clear_background(WHITE);
        let node = board.layout(&text_drawer);
        if !matches!(node, Node::Empty) {
            let t = TaffyNode::from_layout_node(Node::Layout(
                "root".into(),
                Style {
                    size: Size {
                        width: length(screen_width()),
                        height: length(screen_height()),
                    },
                    ..Default::default()
                },
                vec![node],
            ));
            match t {
                Ok(t) => {
                    let rects = TaffyRectNode::new(t);
                    board.update(&rects);
                    board.draw(&text_drawer, &vec2(0.0, 0.0), &rects);
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
        next_frame().await;
    }
}
