use crate::prelude::*;
use macroquad::math::Rect;
use taffy::prelude::*;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TaffyRectNode {
    rect: Rect,
    children: HashMap<String, Box<TaffyRectNode>>,
}
impl TaffyRectNode {
    pub fn new(rect: Rect, children: HashMap<String, Box<TaffyRectNode>>) -> Self {
        Self { rect, children }
    }
    pub fn get_child(&self, s: &str) -> Option<&TaffyRectNode> {
        self.children.get(s).map(|x| &**x)
    }

    pub fn get_all_children(&self) -> &HashMap<String, Box<TaffyRectNode>> {
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
impl LayoutNode {
    pub fn macroquad_rect(&self, taffy: &mut TaffyTree) -> TaffyRectNode {
        TaffyNodeInner::macroquad_rect(self, taffy)
    }
    pub fn root(width: f32, height: f32, wrapped: Self) -> Self {
        LayoutNode::Node(
            "root".to_string(),
            Style {
                size: Size {
                    width: length(width),
                    height: length(height),
                },
                ..Default::default()
            },
            vec![wrapped],
        )
    }
    pub fn screen_root(wrapped: Self) -> Self {
        Self::root(
            macroquad::window::screen_width(),
            macroquad::window::screen_height(),
            wrapped,
        )
    }
}

impl TaffyNodeInner {
    fn _to_macroquad_rect(taffy: &TaffyTree, id: NodeId) -> macroquad::math::Rect {
        let location = Self::get_pos_abs(taffy, id);
        let layout = taffy.layout(id).unwrap();
        macroquad::math::Rect {
            x: location.x,
            y: location.y,
            w: layout.size.width,
            h: layout.size.height,
        }
    }
    fn _to_macroquad(taffy: &TaffyTree, t: TaffyNodeInner) -> Vec<(String, Box<TaffyRectNode>)> {
        if let Some(id) = t.id {
            vec![(
                id,
                Box::new(TaffyRectNode::new(
                    Self::_to_macroquad_rect(taffy, t.node_id),
                    t.children
                        .into_iter()
                        .flat_map(|c| Self::_to_macroquad(taffy, c))
                        .collect(),
                )),
            )]
        } else {
            t.children
                .into_iter()
                .flat_map(|c| Self::_to_macroquad(taffy, c))
                .collect()
        }
    }

    fn to_macroquad(self, taffy: &TaffyTree) -> TaffyRectNode {
        *Self::_to_macroquad(taffy, self)
            .into_iter()
            .nth(0)
            .unwrap()
            .1
    }
    pub fn macroquad_rect(n: &LayoutNode, taffy: &mut TaffyTree) -> TaffyRectNode {
        let taffy_root = Self::new(taffy, n.clone());

        taffy_root.unwrap().to_macroquad(taffy)
    }
}
