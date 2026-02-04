mod components;
mod data;
mod launch;
use crate::{components::*, data::CHARACTERS, launch::*};
use macroquad::prelude::*;

/// To show how taffy_blueprint::Node can be used
/// Each Component has a layout method that returns a Node
/// each parent component build his Node with children Node
/// each component is standalone
#[macroquad::main("cards")]
async fn main() {
    // You can change this to render a sub-component
    let launch = LauchType::Board;

    let td = TextDrawer::new(16).await;
    let mut launch = match launch {
        LauchType::Board => {
            let comp = Box::new(BoardComponent::new(&td, &CHARACTERS.clone()));
            Launch::new(td, comp)
        }
        LauchType::Card => {
            let comp = Box::new(CardComponent::new(&td, CHARACTERS[0].clone()));
            Launch::new(td, comp)
        }
        LauchType::Value => Launch::new(
            td,
            Box::new(ValueComponent::new("Simpathy".into(), 5, 25.0)),
        ),
        LauchType::ValueBar => Launch::new(td, Box::new(ValueBarComponent::new(2))),
        LauchType::Label => Launch::new(td, Box::new(LabelComponent::new("test".into()))),
        LauchType::ValueBarItem => Launch::new(td, Box::new(ValueBarItemComponent::new(true))),
    }
    .await;
    loop {
        clear_background(WHITE);
        match launch.update() {
            UpdateResult::Continue => {}
            UpdateResult::End(character) => {
                println!("choosed {} ({})", character.name, character.class.name());
                return;
            }
        }
        next_frame().await;
    }
}
