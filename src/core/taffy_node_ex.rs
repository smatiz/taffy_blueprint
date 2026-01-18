use std::collections::HashMap;

use crate::core::layout_node_ex::LayoutNodeX;
use taffy::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct TaffyNodeX {
    pub(crate) node_id: NodeId,
    pub(crate) id: Option<String>,
    pub(crate) children: Vec<Self>,
}

impl PartialEq for TaffyNodeX {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.children == other.children
    }
}

// fn assert_send<T: Send>() {}
// fn assert_sync<T: Sync>() {}
#[derive(Clone, PartialEq, Default, Debug)]
pub(crate) struct TaffyNodeXX {
    pub(crate) node_id: Option<NodeId>,
    pub(crate) children: HashMap<String, TaffyNodeXX>,
}

impl TaffyNodeX {
    // fn _to_taffy_x(&self) -> TaffyNodeX {
    //     self.children
    //         .iter()
    //         .flat_map(|n| {
    //             if let Some(id) = n.id {
    //                 n
    //             } else {
    //                 n.
    //             }

    //         })
    //         .collect()
    // }

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

    fn _to_taffy(taffy: &mut TaffyTree, n: LayoutNodeX) -> Option<Self> {
        println!("n:{}", n.debug_without_style());
        let (id, style, items) = n.get_data();

        if items.len() == 0 {
            //// leaf
            if let Some(style) = style {
                match taffy.new_leaf(style) {
                    Ok(node_id) => {
                        if let Some(id) = id {
                            Some(Self {
                                id: Some(id.clone()),
                                node_id,
                                children: vec![],
                            })
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        println!(
                            "Error TaffyNode (id:{}): {}",
                            id.as_deref().unwrap_or("#"),
                            e
                        );
                        return None;
                    }
                }
            } else {
                None
            }
        } else {
            let taffy_items = items
                .into_iter()
                .filter_map(|child| Self::_to_taffy(taffy, child))
                .collect::<Vec<_>>();
            match taffy.new_with_children(
                style.unwrap_or(Style::default()).clone(),
                &taffy_items
                    .iter()
                    .map(|node| node.node_id.clone())
                    .collect::<Vec<_>>(),
            ) {
                Ok(node_id) => {
                    if let Some(id) = id {
                        Some(Self {
                            id: Some(id.clone()),
                            node_id,
                            children: taffy_items,
                        })
                    } else {
                        Some(TaffyNodeX {
                            node_id,
                            id: None,
                            children: taffy_items,
                        })
                    }
                }
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    return None;
                }
            }
        }
    }

    pub fn new(taffy: &mut TaffyTree, n: LayoutNodeX) -> Option<Self> {
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
}

#[cfg(test)]
mod tests {
    use crate::core::layout_node_ex::LayoutNodeX::*;

    use super::*;
    #[test]
    fn test_transformations() {
        let mut taffy = TaffyTree::new();
        let layout_node = Anonym(Style::default(), vec![Anonym(Style::default(), vec![])]);
        let taffy_node_x_b = TaffyNodeX::new(&mut taffy, layout_node);
        let taffy_node_x = TaffyNodeX {
            id: None,
            node_id: NodeId::new(1),
            children: vec![TaffyNodeX {
                id: None,
                node_id: NodeId::new(2),
                children: vec![],
            }],
        };
        // let taffy_node_xx = TaffyNodeXX {
        //     node_id: NodeId::new(1),
        //     children: HashMap::new(),
        // };

        assert_eq!(taffy_node_x, taffy_node_x_b.unwrap());
    }
}
