use super::*;
use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct LabelComponent {
    text: String,
}

impl LabelComponent {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
impl Component for LabelComponent {
    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        let r = text_drawer.measure(&self.text);
        Node::LeafAnonym(h_taffy::style_dimension(r.width, r.height))
    }
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        text_drawer.draw_exact(&self.text, rects.rect().x, rects.rect().y, BLACK);
    }
    fn update(&mut self, _rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}
