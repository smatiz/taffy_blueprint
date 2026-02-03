use macroquad::prelude::*;
use taffy_blueprint::prelude::*;

use crate::text_drawer::TextDrawer;

#[derive(Default, Clone, Debug)]
pub struct Label {
    pub text: String,
}

impl Label {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
        }
    }
    pub fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        let r = text_drawer.measure(&self.text);
        Node::LeafAnonym(h_taffy::style_dimension(r.width, r.height))
    }
    pub fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        self.draw_offset(text_drawer, &Vec2::ZERO, rects);
    }
    pub fn draw_offset(&self, text_drawer: &TextDrawer, offset: &Vec2, rects: &TaffyRectNode<()>) {
        text_drawer.draw_exact(
            &self.text,
            offset.x + rects.rect().x,
            offset.y + rects.rect().y,
            BLACK,
        );
    }
    pub fn draw_color(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>, color: Color) {
        text_drawer.draw_exact(&self.text, rects.rect().x, rects.rect().y, color);
    }
    pub fn draw_color_offset(
        &self,
        text_drawer: &TextDrawer,
        offset: &Vec2,
        rects: &TaffyRectNode<()>,
        color: Color,
    ) {
        text_drawer.draw_exact(
            &self.text,
            offset.x + rects.rect().x,
            offset.y + rects.rect().y,
            color,
        );
    }
}
