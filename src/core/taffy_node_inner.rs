use crate::core::layout_node::LayoutNode;
use std::collections::HashMap;
use taffy::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct TaffyNodeInner {
    pub(crate) node_id: NodeId,
    pub(crate) id: Option<String>,
    pub(crate) children: Vec<Self>,
}

impl TaffyNodeInner {
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

    fn _to_taffy(taffy: &mut TaffyTree, n: LayoutNode) -> Option<Self> {
        let (id, style, items) = n.get_data();

        if items.len() == 0 {
            if let Some(style) = style {
                match taffy.new_leaf(style) {
                    Ok(node_id) => Some(Self {
                        id,
                        node_id,
                        children: vec![],
                    }),
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
                Ok(node_id) => Some(Self {
                    id,
                    node_id,
                    children: taffy_items,
                }),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    return None;
                }
            }
        }
    }

    pub fn new(taffy: &mut TaffyTree, n: LayoutNode) -> Option<Self> {
        if let Some(taffy_root) = Self::_to_taffy(taffy, n) {
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

    #[derive(Debug)]
    struct TaffyNodeTest(TaffyNodeInner);

    // I need to ignore the nodeid field
    impl PartialEq for TaffyNodeTest {
        fn eq(&self, other: &Self) -> bool {
            self.0.id == other.0.id
                && self.0.children.len() == other.0.children.len()
                && self
                    .0
                    .children
                    .iter()
                    .zip(other.0.children.clone())
                    .all(|(x, y)| TaffyNodeTest(x.clone()) == TaffyNodeTest(y))
        }
    }

    use crate::core::layout_node::LayoutNode::*;

    use super::*;
    #[test]
    fn test_anonym() {
        let mut taffy = TaffyTree::new();
        assert_eq!(
            TaffyNodeTest(
                TaffyNodeInner::new(
                    &mut taffy,
                    Anonym(
                        Style::default(),
                        vec![Node("two".to_string(), Style::default(), vec![])],
                    )
                )
                .unwrap()
            ),
            TaffyNodeTest(TaffyNodeInner {
                id: None,
                node_id: NodeId::new(0),
                children: vec![TaffyNodeInner {
                    id: Some("two".to_string()),
                    node_id: NodeId::new(0),
                    children: vec![],
                }],
            }),
        );

        assert_eq!(
            TaffyNodeTest(
                TaffyNodeInner::new(
                    &mut taffy,
                    Node(
                        "one".to_string(),
                        Style::default(),
                        vec![
                            Node(
                                "two".to_string(),
                                Style::default(),
                                vec![
                                    Id("three".to_string(), vec![Empty]),
                                    LeafAnonym(Style::default()),
                                    Leaf("four".to_string(), Style::default())
                                ]
                            ),
                            Empty
                        ],
                    )
                )
                .unwrap()
            ),
            TaffyNodeTest(TaffyNodeInner {
                id: Some("one".to_string()),
                node_id: NodeId::new(0),
                children: vec![TaffyNodeInner {
                    id: Some("two".to_string()),
                    node_id: NodeId::new(0),
                    children: vec![
                        TaffyNodeInner {
                            id: Some("three".to_string()),
                            node_id: NodeId::new(0),
                            children: vec![],
                        },
                        TaffyNodeInner {
                            id: None,
                            node_id: NodeId::new(0),
                            children: vec![],
                        },
                        TaffyNodeInner {
                            id: Some("four".to_string()),
                            node_id: NodeId::new(0),
                            children: vec![],
                        }
                    ],
                },],
            }),
        );
    }
}
