use super::*;
use taffy_blueprint::prelude::*;

#[derive(Debug, Clone)]
pub struct ValueBarComponent {
    value: u8,
}
impl ValueBarComponent {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}
impl Component for ValueBarComponent {
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {}
    fn layout(&self, _text_drawer: &TextDrawer) -> Node<()> {
        Node::Empty
    }
    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}
