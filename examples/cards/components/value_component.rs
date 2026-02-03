use super::*;
use macroquad::prelude::*;
use taffy::{JustifyContent, Style};
use taffy_blueprint::prelude::*;

#[derive(Clone, Debug)]
pub struct ValueComponent {
    pub name: ComponentId<LabelComponent>,
    pub value: ComponentId<ValueBarComponent>,
}

impl ValueComponent {
    pub fn new(name: String, value: u8) -> Self {
        Self {
            name: ComponentId::new("name".into(), LabelComponent::new(name)),
            value: ComponentId::new("value".into(), ValueBarComponent::new(value)),
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
    fn layout(&self, _text_drawer: &TextDrawer) -> Node<()> {
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
    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}
