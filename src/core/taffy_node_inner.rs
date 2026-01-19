use taffy::{prelude::*, Point};

use crate::core::{
    tree_prune::{prune_tree, Prune},
    TaffyNodeRaw,
};

impl Prune for TaffyNodeInner {
    fn keep(&self) -> bool {
        self.id.is_some()
    }

    fn children(&mut self) -> Vec<Self> {
        self.children.drain(0..self.children.len()).collect()
    }

    fn replace_children(self, children: Vec<Self>) -> Self {
        Self { children, ..self }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TaffyNodeInner {
    pub absolute_position: Point<f32>,
    pub layout: Layout,
    pub id: Option<String>,
    pub children: Vec<Self>,
}

impl TaffyNodeInner {
    fn _new(taffy: &TaffyTree, position: Point<f32>, t: TaffyNodeRaw) -> Option<Self> {
        match taffy.layout(t.node_id) {
            Ok(layout) => Some(Self {
                layout: *layout,
                absolute_position: position,
                id: t.id,
                children: t
                    .children
                    .into_iter()
                    .filter_map(|c| Self::_new(taffy, position + layout.location, c))
                    .collect(),
            }),
            Err(e) => {
                println!("Error TaffyNode: {}", e);
                None
            }
        }
    }
    pub fn new(taffy: &TaffyTree, t: TaffyNodeRaw) -> Option<Self> {
        Self::_new(taffy, Point::zero(), t).and_then(prune_tree)
    }
}
