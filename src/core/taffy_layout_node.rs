use std::collections::HashMap;
use taffy::{prelude::*, Point};

use crate::core::{LayoutNode, TaffyNode, TaffyRoot};

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
                            layout: *layout,
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

    pub fn from_taffy_root(tree: TaffyRoot) -> Option<Self> {
        match tree.taffy.layout(tree.node_id) {
            Ok(layout) => Some(Self {
                absolute_position: Point::zero(),
                layout: *layout,
                children: Self::to_hashmap(&tree.taffy, Point::zero(), tree.children),
            }),
            Err(e) => {
                println!("Error TaffyLayoutNode: {}", e);
                None
            }
        }
    }

    pub fn new(layout_node: LayoutNode) -> Option<Self> {
        TaffyRoot::new(layout_node).and_then(|n| Self::from_taffy_root(n))
    }
}
