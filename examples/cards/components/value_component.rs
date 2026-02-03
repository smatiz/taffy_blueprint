use std::alloc::Layout;

use super::*;
use macroquad::prelude::*;
use taffy::{
    prelude::{length, percent},
    AlignItems, JustifyContent, Style,
};
use taffy_blueprint::prelude::*;

#[derive(Clone, Debug)]
pub struct ValueComponent {
    pub name: ComponentId<LabelComponent>,
    pub value: ComponentId<ValueBarComponent>,
    height: f32,
}

impl ValueComponent {
    pub fn new(name: String, value: u8, height: f32) -> Self {
        Self {
            name: ComponentId::new("name".into(), LabelComponent::new(name)),
            value: ComponentId::new("value".into(), ValueBarComponent::new(value)),
            height,
        }
    }
}
impl Component for ValueComponent {
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        self.name
            .item
            .draw(text_drawer, rects.get_child(&self.name.id).unwrap());
        self.value
            .item
            .draw(text_drawer, rects.get_child(&self.value.id).unwrap());
    }
    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        Node::Anonym(
            Style {
                flex_direction: taffy::FlexDirection::Column,
                justify_content: Some(JustifyContent::Center),
                // align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            vec![
                Node::Layout(
                    self.name.id.clone(),
                    Style {
                        size: taffy::Size {
                            width: percent(1.0),
                            height: length(self.height),
                        },
                        ..Default::default()
                    },
                    vec![self.name.item.layout(text_drawer)],
                ),
                Node::Layout(
                    self.value.id.clone(),
                    Style {
                        size: taffy::Size {
                            width: percent(1.0),
                            height: length(self.height),
                        },
                        ..Default::default()
                    },
                    vec![self.value.item.layout(text_drawer)],
                ),
            ],
        )
    }
    fn update(&mut self, _rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}
