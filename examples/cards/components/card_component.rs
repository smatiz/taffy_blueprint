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

impl CardComponent {
    pub fn new(character: Character) -> Self {
        let label_name =
            ComponentId::new("name".into(), LabelComponent::new(character.class.name()));
        let label_class_name = ComponentId::new(
            "class_name".into(),
            LabelComponent::new(character.class.class_name()),
        );
        let values = vec![
            ComponentId::new(
                "".into(),
                ValueComponent::new("Mana".into(), character.mana),
            ),
            ComponentId::new(
                "".into(),
                ValueComponent::new("Strength".into(), character.strength),
            ),
            ComponentId::new(
                "".into(),
                ValueComponent::new("Agility".into(), character.agility),
            ),
            ComponentId::new(
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
    pub fn character(&self) -> &Character {
        &self.character
    }
}
impl Component for CardComponent {
    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult {
        let rc = helper_clickable::search(rects.rect());
        match rc {
            ClickableResult::Hover => self.hover = true,
            ClickableResult::Clicked => {
                return UpdateResult::End;
            }
            ClickableResult::None => self.hover = false,
        }
        UpdateResult::Continue
    }
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
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

    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
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
            std::iter::once(Node::LeafAnonym(Style {
                flex_grow: 1.0,
                ..Default::default()
            }))
            .chain(vec![
                Node::Id(
                    self.label_name.id.clone(),
                    vec![self.label_name.item.layout(text_drawer)],
                ),
                Node::Id(
                    self.label_class_name.id.clone(),
                    vec![self.label_class_name.item.layout(text_drawer)],
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
            .chain(std::iter::once(Node::LeafAnonym(Style {
                flex_grow: 2.0,
                ..Default::default()
            })))
            .collect(),
        )
    }
}
