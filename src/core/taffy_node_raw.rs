use crate::core::{node::Node, TaffyBlueprintError};
use taffy::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct TaffyNodeRaw<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub node_id: NodeId,
    pub id: Option<String>,
    pub children: Vec<Self>,
    pub tag: Option<T>,
}
#[derive(Clone, Debug)]
pub struct TaffyRootRaw<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub taffy: TaffyTree,
    pub root: TaffyNodeRaw<T>,
}

impl<T> TaffyNodeRaw<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn _to_taffy(taffy: &mut TaffyTree, n: Node<T>) -> Result<Self, TaffyBlueprintError> {
        let (id, style, items, tag) = n.get_data();

        if items.is_empty() {
            if let Some(style) = style {
                match taffy.new_leaf(style) {
                    Ok(node_id) => Ok(Self {
                        id,
                        node_id,
                        children: vec![],
                        tag,
                    }),
                    Err(e) => {
                        println!(
                            "Error TaffyNode (id:{}): {}",
                            id.as_deref().unwrap_or("#"),
                            e
                        );
                        Err(TaffyBlueprintError::Taffy(e))
                    }
                }
            } else {
                Err(TaffyBlueprintError::TaffyNodeRaw)
            }
        } else {
            let taffy_items: Result<Vec<_>, _> = items
                .into_iter()
                .map(|child| Self::_to_taffy(taffy, child))
                .collect::<Result<_, _>>();
            let taffy_items: Vec<_> = taffy_items.into_iter().flatten().collect();

            match taffy.new_with_children(
                style.unwrap_or(Style::default()).clone(),
                &taffy_items
                    .iter()
                    .map(|node| node.node_id)
                    .collect::<Vec<_>>(),
            ) {
                Ok(node_id) => Ok(Self {
                    id,
                    node_id,
                    children: taffy_items,
                    tag,
                }),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    Err(TaffyBlueprintError::Taffy(e))
                }
            }
        }
    }
    fn new(taffy: &mut TaffyTree, n: Node<T>) -> Result<Self, TaffyBlueprintError> {
        match Self::_to_taffy(taffy, n) {
            Ok(taffy_root) => match taffy.compute_layout(taffy_root.node_id, Size::MAX_CONTENT) {
                Ok(_) => Ok(taffy_root),
                Err(e) => {
                    println!("Error TaffyNode: {}", e);
                    Err(TaffyBlueprintError::Taffy(e))
                }
            },
            Err(e) => Err(e),
        }
    }
}

impl<T> TaffyRootRaw<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub fn new(n: Node<T>) -> Result<Self, TaffyBlueprintError> {
        let mut taffy = TaffyTree::new();
        TaffyNodeRaw::new(&mut taffy, n).map(|root| Self { taffy, root })
    }
}

#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct TaffyNodeTest(TaffyNodeRaw<()>);

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

    use crate::core::node::Node::*;

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
                    tag: None
                }],
                tag: None
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
                            tag: None
                        },
                        TaffyNodeRaw {
                            id: None,
                            node_id: NodeId::new(0),
                            children: vec![],
                            tag: None
                        },
                        TaffyNodeRaw {
                            id: Some("four".to_string()),
                            node_id: NodeId::new(0),
                            children: vec![],
                            tag: None
                        }
                    ],
                    tag: None
                },],
                tag: None
            }),
        );
    }
}
