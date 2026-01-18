use crate::core::{layout_node::LayoutNode, tree_prune::*, TaffyNodeInner};
use std::{collections::HashMap, f64::NAN};
use taffy::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TaffyNode {
    pub(crate) node_id: NodeId,
    pub(crate) children: HashMap<String, TaffyNode>,
}

impl Prune for TaffyNodeInner {
    type Id = String;
    type Output = TaffyNodeInner;

    fn keep(&self) -> bool {
        self.id.is_some()
    }

    fn children(&self) -> &[Self] {
        &self.children
    }

    fn make_output(n: &Self, children: Vec<Self::Output>) -> Self::Output {
        Self::Output {
            id: n.id,
            node_id: n.node_id,
            children,
        }
    }
}
#[derive(Clone, Default, Debug)]
pub(crate) struct TaffyRoot {
    pub(crate) taffy: TaffyTree,
    pub(crate) node_id: Option<NodeId>,
    pub(crate) children: HashMap<String, TaffyNode>,
}
impl TaffyRoot {
    // pub fn new(n: LayoutNode) -> Self {
    //     let taffy = TaffyTree::new();
    //     let t_inner = TaffyNodeInner::new(&mut taffy, n);

    //     Self {}
    // }
}
