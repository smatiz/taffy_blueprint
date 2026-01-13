use crate::prelude::*;
use taffy::prelude::*;

pub(crate) struct TaffyNode {
    pub(crate) id: NodeId,
    pub(crate) name: String,
    pub(crate) children: Vec<TaffyNode>,
}
impl TaffyNode {
    fn _get_pos_abs(
        taffy: &TaffyTree,
        id: taffy::NodeId,
        v: taffy::Point<f32>,
    ) -> taffy::Point<f32> {
        if let Some(pid) = taffy.parent(id) {
            Self::_get_pos_abs(taffy, pid, v + taffy.layout(id).unwrap().location)
        } else {
            v + taffy.layout(id).unwrap().location
        }
    }
    pub(crate) fn get_pos_abs(taffy: &TaffyTree, id: taffy::NodeId) -> taffy::Point<f32> {
        if let Some(pid) = taffy.parent(id) {
            Self::_get_pos_abs(taffy, pid, taffy.layout(id).unwrap().location)
        } else {
            taffy.layout(id).unwrap().location
        }
    }

    pub(crate) fn _to_taffy(taffy: &mut TaffyTree, n: &LayoutNode) -> TaffyNode {
        if n.children.len() == 0 {
            let id = taffy.new_leaf(n.style.clone()).unwrap();
            TaffyNode {
                name: n.id.clone(),
                id,
                children: vec![],
            }
        } else {
            let ids = n
                .children
                .iter()
                .map(|child| Self::_to_taffy(taffy, child))
                .collect::<Vec<_>>();
            let id = taffy
                .new_with_children(
                    n.style.clone(),
                    &ids.iter().map(|child| child.id.clone()).collect::<Vec<_>>(),
                )
                .unwrap();
            TaffyNode {
                name: n.id.clone(),
                id,
                children: ids,
            }
        }
    }
    fn _to_taffy_simple(taffy: &mut TaffyTree, n: &LayoutNode) -> NodeId {
        if n.children.len() == 0 {
            taffy.new_leaf(n.style.clone()).unwrap()
        } else {
            let ids = n
                .children
                .iter()
                .map(|child| Self::_to_taffy_simple(taffy, child))
                .collect::<Vec<_>>();
            taffy.new_with_children(n.style.clone(), &ids).unwrap()
        }
    }

    pub fn root_size(n: &LayoutNode) -> taffy::Size<f32> {
        let mut taffy = TaffyTree::new();
        let root_id = Self::_to_taffy_simple(&mut taffy, n);
        taffy.compute_layout(root_id, Size::MAX_CONTENT).unwrap();
        let layout = taffy.layout(root_id).unwrap();
        layout.size
    }
}
