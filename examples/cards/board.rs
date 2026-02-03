use crate::{
    card::Card, character::Character, taffy_child::TaffyChild, text_drawer::TextDrawer,
    types::UpdateResult,
};

use super::*;
use macroquad::prelude::*;
use taffy::prelude::*;

/// this is the board where choosing happend
/// there is only one instance of Board (tied to MainBoard)
#[derive(Clone)]
pub struct Board {
    cards: Vec<TaffyChild<Card>>,
}

impl Board {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn start(&mut self, characters: Vec<Character>) {
        self.cards = characters
            .into_iter()
            .enumerate()
            .map(|(i, c)| TaffyChild::new(&format!("{}", i), Card::new(c)))
            .collect();
    }
    fn grow(grow: f32) -> Node<()> {
        Node::LeafAnonym(Style {
            flex_grow: grow,
            ..Default::default()
        })
    }
    pub fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        let max_width = self
            .cards
            .iter()
            .map(|c| {
                let (taffy_tree, node_id) = c.item.layout(text_drawer).to_taffy_tree();
                taffy_tree.layout(node_id).unwrap().content_size.width
            })
            .reduce(f32::max);

        if let Some(max_width) = max_width {
            let original_children = self.cards.iter().map(|card| {
                Node::Anonym(
                    Style {
                        margin: taffy::Rect {
                            left: length(10.0),
                            bottom: length(10.0),
                            top: length(10.0),
                            right: length(10.0),
                        },
                        ..h_taffy::style_auto()
                    },
                    Node::Layout(
                        //**************   card.id
                        card.id.clone(),
                        Style {
                            size: Size {
                                width: length(max_width),
                                height: length(max_width * 4.0 / 3.0),
                            },
                            ..Default::default()
                        },
                        Node::Anonym(
                            Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: Some(JustifyContent::Center),
                                flex_grow: 1.0,
                                ..Default::default()
                            },
                            //**************   card.item
                            card.item.layout(text_drawer).into(),
                        )
                        .into(),
                    )
                    .into(),
                )
            });
            let children = std::iter::once(Self::grow(1.5))
                .chain(original_children)
                .chain(std::iter::once(Self::grow(1.5)))
                .collect();

            Node::Anonym(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![],
                    grid_template_rows: vec![percent(0.2), percent(0.6), percent(0.2)],

                    ..h_taffy::style_full()
                },
                vec![Node::Anonym(
                    Style {
                        grid_row: line(2),
                        display: Display::Flex,
                        ..h_taffy::style_full()
                    },
                    Node::Anonym(
                        Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: Some(JustifyContent::SpaceEvenly),
                            ..h_taffy::style_full()
                        },
                        children,
                    )
                    .into(),
                )],
            )
        } else {
            Node::Empty
        }
    }

    pub fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult<Character> {
        for card in self.cards.iter_mut() {
            if let Some(r) = rects.get_child(&card.id) {
                match card.item.update(r) {
                    UpdateResult::Continue => {}
                    UpdateResult::End(character) => return UpdateResult::End(character),
                }
            }
        }
        UpdateResult::Continue
    }
    pub fn draw(&self, text_drawer: &TextDrawer, offset: &Vec2, rects: &TaffyRectNode<()>) {
        for card in self.cards.iter() {
            if let Some(r) = rects.get_child(&card.id) {
                card.item.draw(text_drawer, offset, r);
            }
        }
    }
}
