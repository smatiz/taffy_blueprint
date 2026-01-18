use std::collections::HashMap;
use taffy::{prelude::*, Point};

use crate::core::{TaffyNode, TaffyRoot};

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyLayoutNode {
    pub layout: Layout,
    pub absolute_position: Point<f32>,
    pub children: HashMap<String, Self>,
}

impl TaffyLayoutNode {
    fn to_hashmap(
        taffy: &TaffyTree,
        position: Point<f32>,
        children: HashMap<String, TaffyNode>,
    ) -> HashMap<String, Self> {
        children
            .into_iter()
            .filter_map(|(name, n)| {
                match taffy.layout(n.node_id) {
                    Ok(layout) => Some((
                        name,
                        Self {
                            layout: layout.clone(),
                            absolute_position: position, //Self::get_pos_abs(taffy, n.node_id),
                            children: Self::to_hashmap(
                                taffy,
                                position + layout.location,
                                n.children,
                            ),
                        },
                    )),
                    Err(e) => {
                        println!("Error TaffyLayoutNode: {}", e);
                        None
                    }
                }
            })
            .collect()
    }

    pub fn new(tree: TaffyRoot) -> Self {
        Self {
            absolute_position: Point::zero(),
            layout: tree.taffy.layout(tree.node_id).unwrap().clone(),
            children: Self::to_hashmap(&tree.taffy, Point::zero(), tree.children),
        }
    }

    // fn _get_pos_abs(
    //     taffy: &TaffyTree,
    //     id: taffy::NodeId,
    //     v: taffy::Point<f32>,
    // ) -> taffy::Point<f32> {
    //     if let Some(pid) = taffy.parent(id) {
    //         Self::_get_pos_abs(taffy, pid, v + taffy.layout(id).unwrap().location)
    //     } else {
    //         v + taffy.layout(id).unwrap().location
    //     }
    // }

    // fn get_pos_abs(taffy: &TaffyTree, id: taffy::NodeId) -> taffy::Point<f32> {
    //     if let Some(pid) = taffy.parent(id) {
    //         Self::_get_pos_abs(taffy, pid, taffy.layout(id).unwrap().location)
    //     } else {
    //         taffy.layout(id).unwrap().location
    //     }
    // }
}
