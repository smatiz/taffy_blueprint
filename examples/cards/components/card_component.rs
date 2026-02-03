use super::*;
use macroquad::prelude::*;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

#[derive(Clone, Debug)]
pub struct CardComponent {
    hover: bool,
    character: Character,
    label_name: ComponentId<LabelComponent>,
    label_class_name: ComponentId<LabelComponent>,
    values: Vec<ComponentId<ValueComponent>>,
}

const ITEMS: [&str; 4] = ["Mana", "Strength", "Agility", "Intelligence"];

impl CardComponent {
    pub fn new(text_drawer: &TextDrawer, character: Character) -> Self {
        let label_name =
            ComponentId::new("name".into(), LabelComponent::new(character.class.name()));
        let label_class_name = ComponentId::new(
            "class_name".into(),
            LabelComponent::new(character.class.class_name()),
        );

        let height = ITEMS
            .iter()
            .map(|item| text_drawer.measure(*item).height)
            .reduce(f32::max)
            .unwrap();

        let values = vec![
            ComponentId::new(
                ITEMS[0].into(),
                ValueComponent::new(ITEMS[0].into(), character.mana, height),
            ),
            ComponentId::new(
                ITEMS[1].into(),
                ValueComponent::new(ITEMS[1].into(), character.strength, height),
            ),
            ComponentId::new(
                ITEMS[2].into(),
                ValueComponent::new(ITEMS[2].into(), character.agility, height),
            ),
            ComponentId::new(
                ITEMS[3].into(),
                ValueComponent::new(ITEMS[3].into(), character.intelligence, height),
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
    pub fn character(&self) -> &Character {
        &self.character
    }
}
impl Component for CardComponent {
    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult {
        if let Some(rects) = rects.get_child("card") {
            let rc = helper_clickable::search(rects.rect());
            match rc {
                ClickableResult::Hover => self.hover = true,
                ClickableResult::Clicked => {
                    return UpdateResult::End;
                }
                ClickableResult::None => self.hover = false,
            }
        }
        UpdateResult::Continue
    }
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        if let Some(rects) = rects.get_child("card") {
            let rect = rects.rect();
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if self.hover { 3.0 } else { 1.0 },
                BLACK,
            );
            self.label_name
                .item
                .draw(text_drawer, rects.get_child(&self.label_name.id).unwrap());
            self.label_class_name.item.draw(
                text_drawer,
                rects.get_child(&self.label_class_name.id).unwrap(),
            );
            for value in self.values.iter() {
                value
                    .item
                    .draw(text_drawer, rects.get_child(&value.id).unwrap());
            }
        }
    }

    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        Node::Layout(
            "card".into(),
            Style {
                ..h_taffy::style_auto()
            },
            vec![Node::Anonym(
                Style {
                    flex_direction: FlexDirection::Column,
                    align_items: Some(AlignItems::Center),
                    // justify_content: Some(JustifyContent::Center),
                    padding: taffy::prelude::Rect {
                        left: length(10.0),
                        right: length(10.0),
                        bottom: length(0.0),
                        top: length(0.0),
                    },
                    ..h_taffy::style_auto()
                },
                std::iter::once(Node::LeafAnonym(Style {
                    flex_grow: 0.2,
                    ..Default::default()
                }))
                .chain(vec![
                    Node::Id(
                        self.label_name.id.clone(),
                        vec![self.label_name.item.layout(text_drawer)],
                    ),
                    Node::LeafAnonym(Style {
                        flex_grow: 0.05,
                        ..Default::default()
                    }),
                    Node::Id(
                        self.label_class_name.id.clone(),
                        vec![self.label_class_name.item.layout(text_drawer)],
                    ),
                    Node::LeafAnonym(Style {
                        flex_grow: 0.1,
                        ..Default::default()
                    }),
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
                // .chain(std::iter::once(Node::LeafAnonym(Style {
                //     flex_grow: 2.0,
                //     ..Default::default()
                // })))
                .collect(),
            )],
        )
    }
}
