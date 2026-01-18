use crate::core::{layout_node::LayoutNode, tree_prune::*, TaffyNodeInner};
use std::collections::HashMap;
use taffy::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyNode {
    pub node_id: NodeId,
    pub children: HashMap<String, TaffyNode>,
}

impl Prune for TaffyNodeInner {
    type Id = String;
    type Output = TaffyNodeInner;

    fn keep(&self) -> bool {
        self.id.is_some()
    }

    fn children(&mut self) -> Vec<Self> {
        self.children.drain(0..self.children.len()).collect()
    }

    fn make_output(n: Self, children: Vec<Self::Output>) -> Self::Output {
        Self::Output {
            id: n.id,
            node_id: n.node_id,
            children,
        }
    }
}
#[derive(Clone, Debug)]
pub struct TaffyRoot {
    pub taffy: TaffyTree,
    pub node_id: NodeId,
    pub children: HashMap<String, TaffyNode>,
}
impl TaffyRoot {
    fn to_hashmap(children: Vec<TaffyNodeInner>) -> HashMap<String, TaffyNode> {
        children
            .into_iter()
            .filter_map(|c| {
                if let Some(id) = c.id {
                    Some((
                        id,
                        TaffyNode {
                            node_id: c.node_id,
                            children: Self::to_hashmap(c.children),
                        },
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn new(n: LayoutNode) -> Option<Self> {
        let mut taffy = TaffyTree::new();
        let t_inner = TaffyNodeInner::new(&mut taffy, n);
        t_inner.and_then(prune_tree).map(|t| Self {
            taffy,
            node_id: t.node_id,
            children: Self::to_hashmap(t.children),
        })
    }
}
