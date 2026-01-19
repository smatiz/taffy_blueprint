use crate::core::{taffy_node_inner::TaffyNodeInner, LayoutNode, TaffyRootRaw};
use std::collections::HashMap;
use taffy::{prelude::*, Point};

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyNode {
    pub absolute_position: Point<f32>,
    pub layout: Layout,
    pub children: HashMap<String, Self>,
}
impl TaffyNode {
    fn _to_hashmap(children: Vec<TaffyNodeInner>) -> HashMap<String, Self> {
        children
            .into_iter()
            .map(|c| {
                (
                    c.id.unwrap(),
                    Self {
                        children: Self::_to_hashmap(c.children),
                        absolute_position: c.absolute_position,
                        layout: c.layout,
                    },
                )
            })
            .collect()
    }

    pub fn new(n: TaffyRootRaw) -> Option<Self> {
        TaffyNodeInner::new(&n.taffy, n.root).map(|t| Self {
            absolute_position: t.absolute_position,
            layout: t.layout,
            children: Self::_to_hashmap(t.children),
        })
    }

    pub fn from_layout_node(n: LayoutNode) -> Option<Self> {
        TaffyRootRaw::new(n).and_then(|n| TaffyNode::new(n))
    }
}
