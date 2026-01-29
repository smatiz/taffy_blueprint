use super::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum Node {
    #[default]
    Empty,
    Layout(String, Style, Vec<Self>),
    Anonym(Style, Vec<Self>),
    Id(String, Vec<Self>),
    Leaf(String, Style),
    LeafAnonym(Style),
}

#[allow(clippy::from_over_into)]
impl Into<Vec<Self>> for Node {
    fn into(self) -> Vec<Self> {
        vec![self]
    }
}

impl Node {
    pub(crate) fn get_data(self) -> (Option<String>, Option<Style>, Vec<Self>) {
        match self {
            Self::Empty => (None, None, vec![]),
            Self::Layout(id, style, items) => (Some(id), Some(style), items),
            Self::Anonym(style, items) => (None, Some(style), items),
            Self::Id(id, items) => (Some(id), None, items),
            Self::Leaf(id, style) => (Some(id), Some(style), vec![]),
            Self::LeafAnonym(style) => (None, Some(style), vec![]),
        }
    }

    pub fn fork(children: Vec<Self>) -> Self {
        Self::Anonym(
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
    #[allow(unused)]
    pub(crate) fn debug_without_style(&self) -> String {
        fn d(id: &str, items: &[Node]) -> String {
            let items: Vec<_> = items
                .iter()
                .map(|item| item.debug_without_style())
                .collect();
            format!("id: {id}, items: {:#?}", items)
        }
        match self {
            Self::Empty => "Empty".to_string(),
            Self::Layout(id, _, items) => format!("Node: {}", d(id, items)),
            Self::Anonym(_, items) => format!("Node: {}", d("#", items)),
            Self::Id(id, items) => format!("Node: {}", d(id, items)),
            Self::Leaf(id, _) => format!("Node: {}", d(id, &[])),
            Self::LeafAnonym(_) => format!("Node: {}", d("#", &[])),
        }
    }

    fn _compute(taffy: &mut TaffyTree, n: Self) -> NodeId {
        let (_, style, children) = n.get_data();
        if children.is_empty() {
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
