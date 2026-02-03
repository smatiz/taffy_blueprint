use super::*;
use crate::{
    character::Character, h_clickable::ClickableResult, label::Label, taffy_child::TaffyChild,
    text_drawer::TextDrawer, types::UpdateResult, value_component::ValueComponent,
};
use macroquad::prelude::*;
use taffy::prelude::*;

#[derive(Clone, Debug)]
pub struct Card {
    hover: bool,
    character: Character,
    label_name: Label,
    label_class_name: Label,
    values: Vec<TaffyChild<ValueComponent>>,
}

impl Card {
    pub fn new(character: Character) -> Self {
        let label_name = Label {
            text: character.class.name(),
        };
        let label_class_name = Label {
            text: character.class.class_name(),
        };
        let values = vec![
            TaffyChild::new(
                "".into(),
                ValueComponent::new("Mana".into(), character.mana),
            ),
            TaffyChild::new(
                "".into(),
                ValueComponent::new("Strength".into(), character.strength),
            ),
            TaffyChild::new(
                "".into(),
                ValueComponent::new("Agility".into(), character.agility),
            ),
            TaffyChild::new(
                "".into(),
                ValueComponent::new("Intelligence".into(), character.intelligence),
            ),
        ];
        Self {
            hover: false,
            label_name,
            label_class_name,
            values,
            character,
        }
    }

    pub fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult<Character> {
        let rc = h_clickable::search(rects.rect());
        match rc {
            ClickableResult::Hover => self.hover = true,
            ClickableResult::Clicked => {
                return UpdateResult::End(self.character.clone());
            }
            ClickableResult::None => self.hover = false,
        }
        UpdateResult::Continue
    }
    pub fn draw(&self, text_drawer: &TextDrawer, offset: &Vec2, rects: &TaffyRectNode<()>) {
        let rect = rects.rect().offset(*offset);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if self.hover { 3.0 } else { 1.0 },
            BLACK,
        );
        self.label_name
            .draw(text_drawer, rects.get_child("name").unwrap());
        self.label_class_name
            .draw(text_drawer, rects.get_child("class_name").unwrap());
        for value in self.values.iter() {
            value
                .item
                .draw(text_drawer, rects.get_child(&value.id).unwrap());
        }
    }

    fn grow(grow: f32) -> Node<()> {
        Node::LeafAnonym(Style {
            flex_grow: grow,
            ..Default::default()
        })
    }
    pub fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        Node::Anonym(
            Style {
                flex_direction: FlexDirection::Column,
                align_items: Some(AlignItems::Center),
                padding: taffy::prelude::Rect {
                    left: length(10.0),
                    right: length(10.0),
                    bottom: length(0.0),
                    top: length(0.0),
                },
                ..h_taffy::style_auto()
            },
            std::iter::once(Self::grow(1.0))
                .chain(vec![
                    Node::Id("name".into(), vec![self.label_name.layout(text_drawer)]),
                    Node::Id(
                        "class_name".into(),
                        vec![self.label_class_name.layout(text_drawer)],
                    ),
                ])
                .chain(self.values.iter().map(|value| {
                    Node::Layout(
                        value.id.clone(),
                        Style {
                            margin: taffy::Rect {
                                left: length(10.0),
                                bottom: length(10.0),
                                top: length(10.0),
                                right: length(10.0),
                            },
                            ..h_taffy::style_auto()
                        },
                        value.item.layout(text_drawer).into(),
                    )
                }))
                .chain(std::iter::once(Self::grow(2.0)))
                .collect(),
        )
    }
}
