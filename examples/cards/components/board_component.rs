use crate::Component;

use super::*;
use macroquad::prelude::*;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

#[derive(Clone)]
pub struct BoardComponent {
    cards: Vec<ComponentId<CardComponent>>,
}

impl BoardComponent {
    pub fn new(text_drawer: &TextDrawer, characters: &[Character]) -> Self {
        Self {
            cards: characters
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    ComponentId::new(
                        &format!("{}", i),
                        CardComponent::new(text_drawer, c.clone()),
                    )
                })
                .collect(),
        }
    }
}

impl Component for BoardComponent {
    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        let max_width = self
            .cards
            .iter()
            .map(|card| {
                let (taffy_tree, node_id) = card.item.layout(text_drawer).to_taffy_tree();
                taffy_tree.layout(node_id).unwrap().content_size.width
            })
            .reduce(f32::max);

        if let Some(max_width) = max_width {
            // let max_width = max_width + 100.0;
            // println!("max_width {}", max_width);
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
                        std::iter::once(Node::LeafAnonym(Style {
                            flex_grow: 1.5,
                            ..Default::default()
                        }))
                        .chain(self.cards.iter().map(|card| {
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
                                    card.id.clone(),
                                    Style {
                                        size: Size {
                                            width: length(max_width),
                                            height: length(400.0),
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
                                        card.item.layout(text_drawer).into(),
                                    )
                                    .into(),
                                )
                                .into(),
                            )
                        }))
                        .chain(std::iter::once(Node::LeafAnonym(Style {
                            flex_grow: 1.5,
                            ..Default::default()
                        })))
                        .collect(),
                    )
                    .into(),
                )],
            )
        } else {
            Node::Empty
        }
    }

    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        for card in self.cards.iter() {
            if let Some(r) = rects.get_child(&card.id) {
                card.item.draw(text_drawer, r);
            }
        }
    }

    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult {
        for card in self.cards.iter_mut() {
            if let Some(r) = rects.get_child(&card.id) {
                match card.item.update(r) {
                    UpdateResult::Continue => {}
                    UpdateResult::End(c) => return UpdateResult::End(c),
                }
            }
        }
        UpdateResult::Continue
    }
}
