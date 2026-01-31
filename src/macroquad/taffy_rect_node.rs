use macroquad::math::Rect;
use std::{collections::HashMap, fmt::Debug};

use crate::core::TaffyNode;

#[derive(Clone, Debug)]
pub struct TaffyRectNode<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    rect: Rect,
    children: HashMap<String, Self>,
    tag: Option<T>,
}
impl<T> TaffyRectNode<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn _rect(t: &TaffyNode<T>) -> Rect {
        Rect {
            x: t.layout.location.x + t.absolute_position.x,
            y: t.layout.location.y + t.absolute_position.y,
            w: t.layout.size.width,
            h: t.layout.size.height,
        }
    }

    fn _children(children: HashMap<String, TaffyNode<T>>) -> HashMap<String, Self> {
        children
            .into_iter()
            .map(|c| {
                (
                    c.0,
                    Self {
                        rect: Self::_rect(&c.1),
                        children: Self::_children(c.1.children),
                        tag: c.1.tag,
                    },
                )
            })
            .collect()
    }

    pub fn new(t: TaffyNode<T>) -> Self {
        Self {
            rect: Self::_rect(&t),
            children: Self::_children(t.children),
            tag: t.tag,
        }
    }
    pub fn get_child(&self, s: &str) -> Option<&Self> {
        self.children.get(s)
    }
    pub fn get_all(&self) -> &HashMap<String, Self> {
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
