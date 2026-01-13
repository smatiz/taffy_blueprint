use super::*;
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
    fn get_pos_abs(taffy: &TaffyTree, id: taffy::NodeId) -> taffy::Point<f32> {
        if let Some(pid) = taffy.parent(id) {
            Self::_get_pos_abs(taffy, pid, taffy.layout(id).unwrap().location)
        } else {
            taffy.layout(id).unwrap().location
        }
    }
    // fn _to_macroquad_rect(taffy: &TaffyTree, id: NodeId) -> macroquad::math::Rect {
    //     let location = Self::get_pos_abs(taffy, id);
    //     let layout = taffy.layout(id).unwrap();
    //     macroquad::math::Rect {
    //         x: location.x,
    //         y: location.y,
    //         w: layout.size.width,
    //         h: layout.size.height,
    //     }
    // }
    // fn _to_macroquad(taffy: &TaffyTree, t: TaffyNode) -> Vec<(String, Box<TaffyRectNode>)> {
    //     if t.name == "" {
    //         t.children
    //             .into_iter()
    //             .flat_map(|c| Self::_to_macroquad(taffy, c))
    //             .collect()
    //     } else {
    //         vec![(
    //             t.name,
    //             Box::new(TaffyRectNode::new(
    //                 Self::_to_macroquad_rect(taffy, t.id),
    //                 t.children
    //                     .into_iter()
    //                     .flat_map(|c| Self::_to_macroquad(taffy, c))
    //                     .collect(),
    //             )),
    //         )]
    //     }
    // }

    fn _to_taffy(taffy: &mut TaffyTree, n: &LayoutNode) -> TaffyNode {
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

    // fn to_macroquad(self, taffy: &TaffyTree) -> TaffyRectNode {
    //     *Self::_to_macroquad(taffy, self)
    //         .into_iter()
    //         .nth(0)
    //         .unwrap()
    //         .1
    // }
    // pub fn macroquad_rect(n: &LayoutNode, taffy: &mut TaffyTree) -> TaffyRectNode {
    //     let taffy_root = Self::_to_taffy(taffy, n);
    //     taffy
    //         .compute_layout(taffy_root.id, Size::MAX_CONTENT)
    //         .unwrap();
    //     taffy_root.to_macroquad(taffy)
    // }

    pub fn root_size(n: &LayoutNode) -> taffy::Size<f32> {
        let mut taffy = TaffyTree::new();
        let root_id = Self::_to_taffy_simple(&mut taffy, n);
        taffy.compute_layout(root_id, Size::MAX_CONTENT).unwrap();
        let layout = taffy.layout(root_id).unwrap();
        layout.size
    }
}
