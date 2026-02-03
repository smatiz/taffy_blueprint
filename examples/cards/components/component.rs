use super::*;
use taffy_blueprint::prelude::*;
pub trait Component {
    fn layout(&self, text_drawer: &TextDrawer) -> Node<()>;
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>);
    fn update(&mut self, rects: &TaffyRectNode<()>) -> UpdateResult;
}
