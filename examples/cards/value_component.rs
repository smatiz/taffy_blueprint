use macroquad::prelude::*;
use taffy::{JustifyContent, Style};
use taffy_blueprint::prelude::*;

use crate::text_drawer::TextDrawer;

#[derive(Clone, Debug)]
pub struct ValueComponent {
    pub name: String,
    pub value: u8,
}

impl ValueComponent {
    pub fn new(name: String, value: u8) -> Self {
        Self { name, value }
    }
    // pub fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult<()> {
    //     UpdateResult::Continue
    // }
    pub fn draw(&self, text_drawer: &TextDrawer, r: &TaffyRectNode<()>) {
        if let Some(rect) = r.get_child("name") {
            let rect = rect.rect();
            text_drawer.draw_exact(&self.name, rect.x, rect.y, BLACK);
        }
        if let Some(rect) = r.get_child("value") {
            let rect = rect.rect();
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, GREEN);
        }
    }
    pub fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        Node::Anonym(
            Style {
                flex_direction: taffy::FlexDirection::Row,
                justify_content: Some(JustifyContent::Start),
                ..Default::default()
            },
            {
                let mut v = vec![Node::Leaf(
                    "name".into(),
                    Style {
                        ..Default::default()
                    },
                )];
                // v.extend((0..6).map(|i| format!("value{}", i)));
                v
            },
        )
    }
}
