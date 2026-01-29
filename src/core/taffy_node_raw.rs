use crate::core::layout::Node;
use taffy::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyNodeRaw {
    pub node_id: NodeId,
    pub id: Option<String>,
    pub children: Vec<Self>,
}
#[derive(Clone, Debug)]
pub struct TaffyRootRaw {
    pub taffy: TaffyTree,
    pub root: TaffyNodeRaw,
}

impl TaffyNodeRaw {
    fn _to_taffy(taffy: &mut TaffyTree, n: Node) -> Option<Self> {
        let (id, style, items) = n.get_data();

        if items.is_empty() {
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
                        None
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
                    .map(|node| node.node_id)
                    .collect::<Vec<_>>(),
            ) {
                Ok(node_id) => Some(Self {
                    id,
                    node_id,
                    children: taffy_items,
                }),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    None
                }
            }
        }
    }
    fn new(taffy: &mut TaffyTree, n: Node) -> Option<Self> {
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

impl TaffyRootRaw {
    pub fn new(n: Node) -> Option<Self> {
        let mut taffy = TaffyTree::new();
        TaffyNodeRaw::new(&mut taffy, n).map(|root| Self { taffy, root })
    }
}

#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct TaffyNodeTest(TaffyNodeRaw);

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

    use crate::core::layout::Node::*;

    use super::*;
    #[test]
    fn test_anonym() {
        let mut taffy = TaffyTree::new();
        assert_eq!(
            TaffyNodeTest(
                TaffyNodeRaw::new(
                    &mut taffy,
                    Anonym(
                        Style::default(),
                        vec![Layout("two".to_string(), Style::default(), vec![])],
                    )
                )
                .unwrap()
            ),
            TaffyNodeTest(TaffyNodeRaw {
                id: None,
                node_id: NodeId::new(0),
                children: vec![TaffyNodeRaw {
                    id: Some("two".to_string()),
                    node_id: NodeId::new(0),
                    children: vec![],
                }],
            }),
        );

        assert_eq!(
            TaffyNodeTest(
                TaffyNodeRaw::new(
                    &mut taffy,
                    Layout(
                        "one".to_string(),
                        Style::default(),
                        vec![
                            Layout(
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
            TaffyNodeTest(TaffyNodeRaw {
                id: Some("one".to_string()),
                node_id: NodeId::new(0),
                children: vec![TaffyNodeRaw {
                    id: Some("two".to_string()),
                    node_id: NodeId::new(0),
                    children: vec![
                        TaffyNodeRaw {
                            id: Some("three".to_string()),
                            node_id: NodeId::new(0),
                            children: vec![],
                        },
                        TaffyNodeRaw {
                            id: None,
                            node_id: NodeId::new(0),
                            children: vec![],
                        },
                        TaffyNodeRaw {
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
