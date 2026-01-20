use super::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum LayoutNode {
    #[default]
    Empty,
    Node(String, Style, Vec<Self>),
    Anonym(Style, Vec<Self>),
    Id(String, Vec<Self>),
    Leaf(String, Style),
    LeafAnonym(Style),
}
impl Into<Vec<Self>> for LayoutNode {
    fn into(self) -> Vec<Self> {
        vec![self]
    }
}

impl LayoutNode {
    pub(crate) fn get_data(self) -> (Option<String>, Option<Style>, Vec<Self>) {
        match self {
            LayoutNode::Empty => (None, None, vec![]),
            LayoutNode::Node(id, style, items) => (Some(id), Some(style), items),
            LayoutNode::Anonym(style, items) => (None, Some(style), items),
            LayoutNode::Id(id, items) => (Some(id), None, items),
            LayoutNode::Leaf(id, style) => (Some(id), Some(style), vec![]),
            LayoutNode::LeafAnonym(style) => (None, Some(style), vec![]),
        }
    }

    pub fn fork(children: Vec<Self>) -> Self {
        LayoutNode::Anonym(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![percent(1.0)],
                grid_template_rows: vec![percent(1.0)],
                ..h_taffy::style_full()
            },
            children
                .into_iter()
                .map(|child| {
                    Self::Anonym(
                        Style {
                            grid_row: line(1),
                            grid_column: line(1),
                            ..h_taffy::style_full()
                        },
                        vec![child],
                    )
                })
                .collect(),
        )
    }
    pub fn debug_without_style(&self) -> String {
        fn d(id: &str, items: &[LayoutNode]) -> String {
            let items: Vec<_> = items
                .iter()
                .map(|item| item.debug_without_style())
                .collect();
            format!("id: {id}, items: {:#?}", items)
        }
        match self {
            LayoutNode::Empty => "Empty".to_string(),
            LayoutNode::Node(id, _, items) => format!("Node: {}", d(id, items)),
            LayoutNode::Anonym(_, items) => format!("Node: {}", d("#", items)),
            LayoutNode::Id(id, items) => format!("Node: {}", d(id, items)),
            LayoutNode::Leaf(id, _) => format!("Node: {}", d(id, &[])),
            LayoutNode::LeafAnonym(_) => format!("Node: {}", d("#", &[])),
        }
    }

    fn _compute(taffy: &mut TaffyTree, n: Self) -> NodeId {
        let (_, style, children) = n.get_data();
        if children.len() == 0 {
            taffy
                .new_leaf(style.unwrap_or(h_taffy::style_auto()))
                .unwrap()
        } else {
            let ids = children
                .into_iter()
                .map(|child| Self::_compute(taffy, child))
                .collect::<Vec<_>>();
            taffy
                .new_with_children(style.unwrap_or(h_taffy::style_auto()), &ids)
                .unwrap()
        }
    }
    pub fn to_taffy_tree(self) -> (TaffyTree, NodeId) {
        let mut taffy = TaffyTree::new();
        let root_id = Self::_compute(&mut taffy, self);
        taffy.compute_layout(root_id, Size::MAX_CONTENT).unwrap();
        (taffy, root_id)
    }
}
