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
// impl From<Vec<Self>> for LayoutNode {
//     fn from(value: Vec<LayoutNode>) -> Self {
//         value.into_iter().next().unwrap()
//     }
// }

impl LayoutNode {
    // pub fn new(id: String, style: Style, children: Vec<Self>) -> LayoutNode {
    //     LayoutNode::Node(id, style, children)
    // }

    // pub fn anonym(style: Style, children: Vec<Self>) -> LayoutNode {
    //     LayoutNode::Anonym(style, children)
    // }
    // pub fn id(id: String, children: Vec<Self>) -> LayoutNode {
    //     LayoutNode::Id(id, children)
    // }

    // TODO remove?
    pub fn grow(grow: f32) -> Self {
        Self::LeafAnonym(Style {
            flex_grow: grow,
            ..Default::default()
        })
    }
    // TODO remove?
    fn _to_taffy_simple(taffy: &mut TaffyTree, n: &LayoutNode) -> NodeId {
        if n.get_children_xxx().len() == 0 {
            taffy
                .new_leaf(n.get_style_xxx().unwrap_or(&h_taffy::style_auto()).clone())
                .unwrap()
        } else {
            let ids = n
                .get_children_xxx()
                .iter()
                .map(|child| Self::_to_taffy_simple(taffy, child))
                .collect::<Vec<_>>();
            taffy
                .new_with_children(
                    n.get_style_xxx().unwrap_or(&h_taffy::style_auto()).clone(),
                    &ids,
                )
                .unwrap()
        }
    }
    // TODO remove?
    pub fn root_size(&self) -> taffy::Size<f32> {
        let mut taffy = TaffyTree::new();
        let root_id = Self::_to_taffy_simple(&mut taffy, self);
        taffy.compute_layout(root_id, Size::MAX_CONTENT).unwrap();
        let layout = taffy.layout(root_id).unwrap();
        layout.size
    }
    // pub fn root_size(&self) -> taffy::Size<f32> {
    //     taffy::Size::zero()
    //     // TaffyNode::root_size(self)
    // }
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

    // TODO remove?
    pub fn get_children_xxx(&self) -> &[Self] {
        match self {
            LayoutNode::Empty => &[],
            LayoutNode::Node(_, _, items) => items,
            LayoutNode::Anonym(_, items) => items,
            LayoutNode::Id(_, items) => items,
            LayoutNode::Leaf(_, _) => &[],
            LayoutNode::LeafAnonym(_) => &[],
        }
    }

    // TODO remove?
    pub fn get_style_xxx(&self) -> Option<&Style> {
        match self {
            LayoutNode::Empty => None,
            LayoutNode::Node(_, style, _) => Some(style),
            LayoutNode::Anonym(style, _) => Some(style),
            LayoutNode::Id(_, _) => None,
            LayoutNode::Leaf(_, style) => Some(style),
            LayoutNode::LeafAnonym(style) => Some(style),
        }
    }

    // TODO remove?
    pub fn get_id_xxx(&self) -> Option<&String> {
        match self {
            LayoutNode::Empty => None,
            LayoutNode::Node(id, _, _) => Some(id),
            LayoutNode::Anonym(_, _) => None,
            LayoutNode::Id(id, _) => Some(id),
            LayoutNode::Leaf(id, _) => Some(id),
            LayoutNode::LeafAnonym(_) => None,
        }
    }

    // get_style_xxx().unwrap_or(&h_taffy::style_auto())
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
}
