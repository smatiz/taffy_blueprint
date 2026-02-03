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
    let launch = LauchType::ValueBar;

    let td = TextDrawer::new(16).await;
    let mut launch = match launch {
        LauchType::Board => Launch::new(td, Box::new(BoardComponent::new(CHARACTERS))),
        LauchType::Card => Launch::new(td, Box::new(CardComponent::new(CHARACTERS[0].clone()))),
        LauchType::Value => Launch::new(td, Box::new(ValueComponent::new("Simpathy".into(), 5))),
        LauchType::ValueBar => Launch::new(td, Box::new(ValueBarComponent::new(3))),
        LauchType::Label => Launch::new(td, Box::new(LabelComponent::new("test".into()))),
    }
    .await;
    loop {
        clear_background(WHITE);
        launch.update();
        next_frame().await;
    }
}
