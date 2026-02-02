use super::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    #[default]
    Empty,
    Layout(String, Style, Vec<Self>),
    Anonym(Style, Vec<Self>),
    Id(String, Vec<Self>),
    Leaf(String, Style),
    LeafAnonym(Style),
    Debug(Box<Self>, T),
}

impl<T> Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    pub(crate) fn get_data(self) -> (Option<String>, Option<Style>, Vec<Self>, Option<T>) {
        match self {
            Self::Empty => (None, None, vec![], None),
            Self::Layout(id, style, items) => (Some(id), Some(style), items, None),
            Self::Anonym(style, items) => (None, Some(style), items, None),
            Self::Id(id, items) => (Some(id), None, items, None),
            Self::Leaf(id, style) => (Some(id), Some(style), vec![], None),
            Self::LeafAnonym(style) => (None, Some(style), vec![], None),
            Self::Debug(node, tag) => {
                let data = node.get_data();
                (data.0, data.1, data.2, Some(tag))
            }
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
    fn _d(id: &str, items: &[Self]) -> String {
        let items: Vec<_> = items
            .iter()
            .map(|item| item.debug_without_style())
            .collect();
        format!("id: {id}, items: {:#?}", items)
    }
    #[allow(unused)]
    pub(crate) fn debug_without_style(&self) -> String {
        match self {
            Self::Empty => "Empty".to_string(),
            Self::Layout(id, _, items) => format!("Node: {}", Self::_d(id, items)),
            Self::Anonym(_, items) => format!("Node: {}", Self::_d("#", items)),
            Self::Id(id, items) => format!("Node: {}", Self::_d(id, items)),
            Self::Leaf(id, _) => format!("Node: {}", Self::_d(id, &[])),
            Self::LeafAnonym(_) => format!("Node: {}", Self::_d("#", &[])),
            Self::Debug(node, debug_label) => todo!(),
        }
    }

    fn _compute(taffy: &mut TaffyTree, n: Self) -> NodeId {
        let (_, style, children, _tag) = n.get_data();
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
#[allow(clippy::from_over_into)]
impl<T> Into<Vec<Self>> for Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn into(self) -> Vec<Self> {
        vec![self]
    }
}
impl<T> PartialEq for Node<T>
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Layout(l0, l1, l2), Self::Layout(r0, r1, r2)) => {
                l0 == r0 && compare_style(l1, r1) && l2 == r2
            }
            (Self::Anonym(l0, l1), Self::Anonym(r0, r1)) => compare_style(l0, r0) && l1 == r1,
            (Self::Id(l0, l1), Self::Id(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Leaf(l0, l1), Self::Leaf(r0, r1)) => l0 == r0 && compare_style(l1, r1),
            (Self::LeafAnonym(l0), Self::LeafAnonym(r0)) => compare_style(l0, r0),
            _ => false,
        }
    }
}
#[cfg(test)]
use crate::core::node_test::compare_style;
#[cfg(not(test))]
#[inline(always)]
fn compare_style(s1: &Style, s2: &Style) -> bool {
    s1 == s2
}
