use crate::h_taffy::style_auto;
use serde::Serialize;

use super::*;
use std::collections::HashMap;

use taffy::prelude::*;

#[derive(Clone, Debug)]
pub struct TaffyRectNode {
    rect: Rect<f32>,
    children: HashMap<String, Box<TaffyRectNode>>,
}
impl TaffyRectNode {
    pub fn new(rect: Rect<f32>, children: HashMap<String, Box<TaffyRectNode>>) -> Self {
        Self { rect, children }
    }
    pub fn get_child(&self, s: &str) -> Option<&TaffyRectNode> {
        self.children.get(s).map(|x| &**x)
    }
    pub fn get_all(&self) -> &HashMap<String, Box<TaffyRectNode>> {
        &self.children
    }
    pub fn rect(&self) -> &Rect<f32> {
        &self.rect
    }
    fn _print(n: &Self, name: &str, depth: usize) {
        println!(
            "{} >{}< {:?} ({})",
            "-".repeat(depth),
            name,
            n.rect,
            n.children.len()
        );
        for (name, n) in n.children.iter() {
            Self::_print(n, name, depth + 1);
        }
    }

    pub fn print(&self, name: &str) {
        Self::_print(self, name, 0);
    }
}

#[derive(Default, Clone, Serialize)]
pub struct LayoutNode {
    pub(crate) style: Style,
    pub(crate) id: String,
    pub(crate) children: Vec<Self>,
}

// struct StyleWrapper<'a>(&'a Style);
// impl<'a> StyleWrapper<'a> {
//     fn _print(diff: &Diff, depth: usize, f: &mut DebugStruct) {
//         match diff {
//             Diff::Added(_) => {}
//             Diff::Removed(_) => {}
//             Diff::Changed { old, new } => {
//                 let _ = f.field(&format!("{} {} => {}", " ".repeat(depth + 3), old, new), &1);
//             }
//             Diff::Children(btree_map) => {
//                 for (s, diff) in btree_map {
//                     let _ = f.field(&format!("{} {}:", "-".repeat(depth + 1), s), &1);
//                     Self::_print(diff, depth + 2, f);
//                 }
//             }
//         }
//     }
// }

// impl<'a> Debug for StyleWrapper<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut fx = f.debug_struct("Style");
//         let default_style = Style::default();
//         let diff = Diff::new(&default_style, self.0);
//         Self::_print(&diff, 0, &mut fx);

//         fx.finish()
//         //     .field("style", self.0.)
//         //     .field("id", &self.id)
//         //     .field("children", &self.children)
//         //     .finish()
//     }
// }
// impl Debug for LayoutNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("LayoutNode")
//             .field("id", &self.id)
//             .field("style", &StyleWrapper(&self.style))
//             .field("children", &self.children)
//             .finish()
//     }
// }

impl LayoutNode {
    // pub fn macroquad_rect(&self, taffy: &mut TaffyTree) -> TaffyRectNode {
    //     TaffyNode::macroquad_rect(self, taffy)
    // }
    pub fn root_size(&self) -> taffy::Size<f32> {
        TaffyNode::root_size(self)
    }

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
    pub fn named(id: String, child: Self) -> Self {
        Self::new(id, style_auto(), vec![child])
    }

    pub fn or_empty(item: Option<Self>) -> Self {
        item.unwrap_or(Self::empty())
    }
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
    pub fn leaf_anonym(style: Style) -> Self {
        Self::anonym(style, vec![])
    }

    // pub fn screen_root(wrapped: Self) -> Self {
    //     LayoutNode::new(
    //         "screen".to_string(),
    //         Style {
    //             size: Size {
    //                 width: length(macroquad::window::screen_width()),
    //                 height: length(macroquad::window::screen_height()),
    //             },
    //             ..Default::default()
    //         },
    //         vec![wrapped],
    //     )
    // }
    pub fn dimension(width: Dimension, height: Dimension) -> Self {
        Self {
            style: Style {
                size: Size { width, height },
                ..Default::default()
            },
            ..Default::default()
        }
    }
    pub fn grow(grow: f32) -> Self {
        Self {
            style: Style {
                flex_grow: grow,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
