use crate::core::{taffy_node_inner::TaffyNodeInner, Node, TaffyBlueprintError, TaffyRootRaw};
use std::collections::HashMap;
use taffy::Point;

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyNode<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub absolute_position: Point<f32>,
    pub layout: taffy::Layout,
    pub children: HashMap<String, Self>,
    pub tag: Option<T>,
}
impl<T> TaffyNode<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn _to_hashmap(children: Vec<TaffyNodeInner<T>>) -> HashMap<String, Self> {
        children
            .into_iter()
            .map(|c| {
                (
                    c.id.unwrap(),
                    Self {
                        children: Self::_to_hashmap(c.children),
                        absolute_position: c.absolute_position,
                        layout: c.layout,
                        tag: c.tag,
                    },
                )
            })
            .collect()
    }

    pub fn new(n: TaffyRootRaw<T>) -> Result<Self, TaffyBlueprintError> {
        TaffyNodeInner::new(&n.taffy, n.root).map(|t| Self {
            absolute_position: t.absolute_position,
            layout: t.layout,
            children: Self::_to_hashmap(t.children),
            tag: t.tag,
        })
    }

    pub fn from_layout_node(n: Node<T>) -> Result<Self, TaffyBlueprintError> {
        TaffyRootRaw::new(n).and_then(Self::new)
    }
}
