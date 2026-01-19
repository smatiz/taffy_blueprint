use macroquad::math::Rect;
use std::{collections::HashMap, fmt::Debug};

use crate::core::TaffyNode;

#[derive(Clone, Debug)]
pub struct TaffyRectNode {
    rect: Rect,
    children: HashMap<String, TaffyRectNode>,
}
impl TaffyRectNode {
    fn _rect(t: &TaffyNode) -> Rect {
        Rect {
            x: t.layout.location.x + t.absolute_position.x,
            y: t.layout.location.y + t.absolute_position.y,
            w: t.layout.size.width,
            h: t.layout.size.height,
        }
    }

    fn _children(children: HashMap<String, TaffyNode>) -> HashMap<String, Self> {
        children
            .into_iter()
            .map(|c| {
                (
                    c.0,
                    Self {
                        rect: Self::_rect(&c.1),
                        children: Self::_children(c.1.children),
                    },
                )
            })
            .collect()
    }

    pub fn new(t: TaffyNode) -> Self {
        Self {
            rect: Self::_rect(&t),
            children: Self::_children(t.children),
        }
    }
    pub fn get_child(&self, s: &str) -> Option<&TaffyRectNode> {
        self.children.get(s)
    }
    pub fn get_all(&self) -> &HashMap<String, TaffyRectNode> {
        &self.children
    }
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
    fn _print(n: &Self, name: &str, depth: usize) {
        println!(
            "{} >{}< {:?} ({})",
            "-".repeat(depth),
            name,
            n.rect,
            n.children.len()
        );
        for (name, n) in n.children.iter() {
            Self::_print(n, name, depth + 1);
        }
    }

    pub fn print(&self, name: &str) {
        Self::_print(self, name, 0);
    }
}
