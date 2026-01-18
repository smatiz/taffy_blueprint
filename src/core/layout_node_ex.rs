use super::*;
use crate::core::h_taffy::style_auto;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub enum LayoutNodeX {
    #[default]
    Empty,
    // Single(String, Style, Box<LayoutNodeX>),
    Node(String, Style, Vec<Self>),
    Anonym(Style, Vec<Self>),
    // SingleAnonym(Style, Box<LayoutNodeX>),
    Id(String, Vec<Self>),
    Leaf(String, Style),
    LeafAnonym(Style),
}

// impl std::fmt::Debug for LayoutNodeX {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Empty => write!(f, "Empty"),
//             Self::Node(arg0, arg1, arg2) => f.debug_tuple("Node").field(arg0).field(arg2).finish(),
//             Self::Anonym(arg0, arg1) => f.debug_tuple("Anonym").field(arg1).finish(),
//             Self::Id(arg0, arg1) => f.debug_tuple("Id").field(arg0).field(arg1).finish(),
//             Self::Leaf(arg0, arg1) => f.debug_tuple("Leaf").field(arg0).finish(),
//             Self::LeafAnonym(arg0) => f.debug_tuple("LeafAnonym").field(arg0).finish(),
//         }
//     }
// }
impl LayoutNodeX {
    pub fn get_data(self) -> (Option<String>, Option<Style>, Vec<Self>) {
        match self {
            LayoutNodeX::Empty => todo!(),
            LayoutNodeX::Node(id, style, items) => (Some(id), Some(style), items),
            LayoutNodeX::Anonym(style, items) => (None, Some(style), items),
            LayoutNodeX::Id(id, items) => (Some(id), None, items),
            LayoutNodeX::Leaf(id, style) => (Some(id), Some(style), vec![]),
            LayoutNodeX::LeafAnonym(style) => (None, Some(style), vec![]),
        }
    }

    pub fn debug_without_style(&self) -> String {
        return "???????????".to_string();
        fn d(id: &str, items: &Vec<LayoutNodeX>) -> String {
            let items = items.into_iter().map(|item| item.debug_without_style());
            format!("id: {id}, items: {:#?}", items)
        }
        match self {
            LayoutNodeX::Empty => "Empty".to_string(),
            LayoutNodeX::Node(id, _, items) => format!("Node: {}", d(id, items)),
            LayoutNodeX::Anonym(_, items) => format!("Node: {}", d("#", items)),
            LayoutNodeX::Id(id, items) => format!("Node: {}", d(id, items)),
            LayoutNodeX::Leaf(id, _) => format!("Node: {}", d(id, &vec![])),
            LayoutNodeX::LeafAnonym(_) => format!("Node: {}", d("#", &vec![])),
        }
    }

    // pub fn root_size(&self) -> taffy::Size<f32> {
    //     TaffyNode::root_size(self)
    // }

    // pub fn new(id: String, style: Style, children: Vec<Self>) -> Self {
    //     Self {
    //         id,
    //         style,
    //         children,
    //     }
    // }

    // pub fn empty() -> Self {
    //     Self::new("".to_string(), h_taffy::style_auto(), vec![])
    // }
    // pub fn single(id: String, style: Style, child: Self) -> Self {
    //     Self::new(id, style, vec![child])
    // }

    // /// This is a node used normally to give a name to an anonym Node
    // pub fn named(id: String, child: Self) -> Self {
    //     Self::new(id, style_auto(), vec![child])
    // }

    // // pub fn or_empty(item: Option<Self>) -> Self {
    // //     item.unwrap_or(Self::empty())
    // // }

    // pub fn single_anonym(style: Style, child: Self) -> Self {
    //     Self::single("".to_string(), style, child)
    // }
    // pub fn anonym(style: Style, children: Vec<Self>) -> Self {
    //     Self::new("".to_string(), style, children)
    // }

    pub fn fork(children: Vec<Self>) -> Self {
        LayoutNodeX::Anonym(
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

    // pub fn leaf(id: String, style: Style) -> Self {
    //     Self {
    //         id,
    //         style,
    //         children: vec![],
    //     }
    // }
    // pub fn leaf_anonym(style: Style) -> Self {
    //     Self::anonym(style, vec![])
    // }

    // pub fn dimension(width: Dimension, height: Dimension) -> Self {
    //     Self {
    //         style: Style {
    //             size: Size { width, height },
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     }
    // }
    // pub fn grow(grow: f32) -> Self {
    //     Self {
    //         style: Style {
    //             flex_grow: grow,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     }
    // }
}
