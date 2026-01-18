use crate::prelude::*;
use taffy::prelude::*;

pub(crate) struct TaffyNode {
    pub(crate) node_id: NodeId,
    pub(crate) id: String,
    pub(crate) children: Vec<Self>,
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

    fn _to_taffy(taffy: &mut TaffyTree, n: &LayoutNode) -> Option<Self> {
        if n.children.len() == 0 {
            match taffy.new_leaf(n.style.clone()) {
                Ok(id) => Some(Self {
                    id: n.id.clone(),
                    node_id: id,
                    children: vec![],
                }),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    return None;
                }
            }
        } else {
            let ids = n
                .children
                .iter()
                .filter_map(|child| Self::_to_taffy(taffy, child))
                .collect::<Vec<_>>();
            match taffy.new_with_children(
                n.style.clone(),
                &ids.iter()
                    .map(|child| child.node_id.clone())
                    .collect::<Vec<_>>(),
            ) {
                Ok(id) => Some(Self {
                    id: n.id.clone(),
                    node_id: id,
                    children: ids,
                }),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    return None;
                }
            }
        }
    }

    pub fn new(taffy: &mut TaffyTree, n: &LayoutNode) -> Option<Self> {
        let taffy_root = Self::_to_taffy(taffy, n);

        if let Some(taffy_root) = taffy_root {
            match taffy.compute_layout(taffy_root.node_id, Size::MAX_CONTENT) {
                Ok(_) => Some(taffy_root),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    // fn _to_taffy_simple(taffy: &mut TaffyTree, n: &LayoutNode) -> NodeId {
    //     if n.children.len() == 0 {
    //         taffy.new_leaf(n.style.clone()).unwrap()
    //     } else {
    //         let ids = n
    //             .children
    //             .iter()
    //             .map(|child| Self::_to_taffy_simple(taffy, child))
    //             .collect::<Vec<_>>();
    //         taffy.new_with_children(n.style.clone(), &ids).unwrap()
    //     }
    // }

    // pub fn root_size(n: &LayoutNode) -> taffy::Size<f32> {
    //     let mut taffy = TaffyTree::new();
    //     let root_id = Self::_to_taffy_simple(&mut taffy, n);
    //     taffy.compute_layout(root_id, Size::MAX_CONTENT).unwrap();
    //     let layout = taffy.layout(root_id).unwrap();
    //     layout.size
    // }
}
