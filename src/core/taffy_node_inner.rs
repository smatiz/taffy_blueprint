use taffy::{prelude::*, Point};

use crate::core::{
    tree_prune::{prune_tree, Prune},
    TaffyBlueprintError, TaffyNodeRaw,
};

impl<T> Prune for TaffyNodeInner<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn keep(&self) -> bool {
        self.id.is_some()
    }

    fn children<'a>(&'a self) -> &'a Vec<Self> {
        &self.children
    }

    fn new(&self, children: Vec<Self>) -> Self {
        Self {
            children,
            ..self.clone()
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TaffyNodeInner<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub absolute_position: Point<f32>,
    pub layout: Layout,
    pub id: Option<String>,
    pub children: Vec<Self>,
    pub tag: Option<T>,
}

impl<T> TaffyNodeInner<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn _new(
        taffy: &TaffyTree,
        position: Point<f32>,
        t: TaffyNodeRaw<T>,
    ) -> Result<Self, TaffyBlueprintError> {
        match taffy.layout(t.node_id) {
            Ok(layout) => {
                let children: Result<Vec<Self>, TaffyBlueprintError> = t
                    .children
                    .into_iter()
                    .map(|c| Self::_new(taffy, position + layout.location, c))
                    .collect();
                match children {
                    Ok(children) => Ok(Self {
                        layout: *layout,
                        absolute_position: position,
                        id: t.id,
                        children,
                        tag: t.tag,
                    }),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(TaffyBlueprintError::Taffy(e)),
        }
    }
    pub fn new(taffy: &TaffyTree, t: TaffyNodeRaw<T>) -> Result<Self, TaffyBlueprintError> {
        Self::_new(taffy, Point::zero(), t).and_then(prune_tree)
    }
}
