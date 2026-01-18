use super::*;
use crate::core::h_taffy::style_auto;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct LayoutNode {
    pub(crate) style: Style,
    pub(crate) id: String,
    pub(crate) children: Vec<Self>,
}

impl LayoutNode {
    // pub fn root_size(&self) -> taffy::Size<f32> {
    //     TaffyNode::root_size(self)
    // }

    pub fn new(id: String, style: Style, children: Vec<Self>) -> Self {
        Self {
            id,
            style,
            children,
        }
    }

    pub fn empty() -> Self {
        Self::new("".to_string(), h_taffy::style_auto(), vec![])
    }
    pub fn single(id: String, style: Style, child: Self) -> Self {
        Self::new(id, style, vec![child])
    }

    /// This is a node used normally to give a name to an anonym Node
    pub fn named(id: String, child: Self) -> Self {
        Self::new(id, style_auto(), vec![child])
    }

    // pub fn or_empty(item: Option<Self>) -> Self {
    //     item.unwrap_or(Self::empty())
    // }

    pub fn single_anonym(style: Style, child: Self) -> Self {
        Self::single("".to_string(), style, child)
    }
    pub fn anonym(style: Style, children: Vec<Self>) -> Self {
        Self::new("".to_string(), style, children)
    }

    pub fn fork(children: Vec<Self>) -> Self {
        LayoutNode::anonym(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![percent(1.0)],
                grid_template_rows: vec![percent(1.0)],
                ..h_taffy::style_full()
            },
            children
                .into_iter()
                .map(|child| {
                    Self::single_anonym(
                        Style {
                            grid_row: line(1),
                            grid_column: line(1),
                            ..h_taffy::style_full()
                        },
                        child,
                    )
                })
                .collect(),
        )
    }

    pub fn leaf(id: String, style: Style) -> Self {
        Self {
            id,
            style,
            children: vec![],
        }
    }
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
