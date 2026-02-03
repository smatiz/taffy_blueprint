use super::*;
use macroquad::{
    color::{BLACK, GREEN},
    shapes::{draw_rectangle, draw_rectangle_lines},
};
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

const NAME: &str = "active";

#[derive(Debug, Clone)]
pub struct ValueBarItemComponent {
    active: bool,
}
impl ValueBarItemComponent {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}
impl Component for ValueBarItemComponent {
    fn draw(&self, _text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        if self.active {
            if let Some(rect) = rects.get_child(NAME).map(|c| c.rect()) {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, GREEN);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
            }
        }
    }
    fn layout(&self, _text_drawer: &TextDrawer) -> Node<()> {
        Node::Anonym(
            Style {
                size: Size {
                    width: percent(1.0),
                    height: percent(1.0),
                },
                padding: Rect {
                    left: percent(0.1),
                    right: percent(0.1),
                    top: percent(0.1),
                    bottom: percent(0.1),
                },
                ..Default::default()
            },
            vec![Node::Leaf(
                NAME.into(),
                Style {
                    size: Size {
                        width: percent(1.0),
                        height: percent(1.0),
                    },
                    ..Default::default()
                },
            )],
        )
    }
    fn update(&mut self, _rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}
