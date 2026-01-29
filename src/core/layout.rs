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

impl PartialEq for Node {
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

#[cfg(not(test))]
#[inline(always)]
fn compare_style(s1: &Style, s2: &Style) -> bool {
    s1 == s2
}
#[cfg(test)]
fn compare_style(s1: &Style, s2: &Style) -> bool {
    if s1.display != s2.display {
        println!("display: {:?} \n!=\n {:?}", s1.display, s2.display);
        return false;
    }
    if s1.item_is_table != s2.item_is_table {
        println!(
            "item_is_table: {:?} \n!=\n {:?}",
            s1.item_is_table, s2.item_is_table
        );
        return false;
    }
    if s1.item_is_replaced != s2.item_is_replaced {
        println!(
            "item_is_replaced: {:?} \n!=\n {:?}",
            s1.item_is_replaced, s2.item_is_replaced
        );
        return false;
    }
    if s1.box_sizing != s2.box_sizing {
        println!("box_sizing: {:?} \n!=\n {:?}", s1.box_sizing, s2.box_sizing);
        return false;
    }
    if s1.overflow != s2.overflow {
        println!("overflow: {:?} \n!=\n {:?}", s1.overflow, s2.overflow);
        return false;
    }
    if s1.scrollbar_width != s2.scrollbar_width {
        println!(
            "scrollbar_width: {:?} \n!=\n {:?}",
            s1.scrollbar_width, s2.scrollbar_width
        );
        return false;
    }
    if s1.position != s2.position {
        println!("position: {:?} \n!=\n {:?}", s1.position, s2.position);
        return false;
    }
    if s1.inset != s2.inset {
        println!("inset: {:?} \n!=\n {:?}", s1.inset, s2.inset);
        return false;
    }
    if s1.size != s2.size {
        println!("size: {:?} \n!=\n {:?}", s1.size, s2.size);
        return false;
    }
    if s1.min_size != s2.min_size {
        println!("min_size: {:?} \n!=\n {:?}", s1.min_size, s2.min_size);
        return false;
    }
    if s1.max_size != s2.max_size {
        println!("max_size: {:?} \n!=\n {:?}", s1.max_size, s2.max_size);
        return false;
    }
    if s1.aspect_ratio != s2.aspect_ratio {
        println!(
            "aspect_ratio: {:?} \n!=\n {:?}",
            s1.aspect_ratio, s2.aspect_ratio
        );
        return false;
    }
    if s1.margin != s2.margin {
        println!("margin: {:?} \n!=\n {:?}", s1.margin, s2.margin);
        return false;
    }
    if s1.padding != s2.padding {
        println!("padding: {:?} \n!=\n {:?}", s1.padding, s2.padding);
        return false;
    }
    if s1.border != s2.border {
        println!("border: {:?} \n!=\n {:?}", s1.border, s2.border);
        return false;
    }
    if s1.align_items != s2.align_items {
        println!(
            "align_items: {:?} \n!=\n {:?}",
            s1.align_items, s2.align_items
        );
        return false;
    }
    if s1.align_self != s2.align_self {
        println!("align_self: {:?} \n!=\n {:?}", s1.align_self, s2.align_self);
        return false;
    }
    if s1.justify_items != s2.justify_items {
        println!(
            "justify_items: {:?} \n!=\n {:?}",
            s1.justify_items, s2.justify_items
        );
        return false;
    }
    if s1.justify_self != s2.justify_self {
        println!(
            "justify_self: {:?} \n!=\n {:?}",
            s1.justify_self, s2.justify_self
        );
        return false;
    }
    if s1.align_content != s2.align_content {
        println!(
            "align_content: {:?} \n!=\n {:?}",
            s1.align_content, s2.align_content
        );
        return false;
    }
    if s1.justify_content != s2.justify_content {
        println!(
            "justify_content: {:?} \n!=\n {:?}",
            s1.justify_content, s2.justify_content
        );
        return false;
    }
    if s1.gap != s2.gap {
        println!("gap: {:?} \n!=\n {:?}", s1.gap, s2.gap);
        return false;
    }
    if s1.text_align != s2.text_align {
        println!("text_align: {:?} \n!=\n {:?}", s1.text_align, s2.text_align);
        return false;
    }
    if s1.flex_direction != s2.flex_direction {
        println!(
            "flex_direction: {:?} \n!=\n {:?}",
            s1.flex_direction, s2.flex_direction
        );
        return false;
    }
    if s1.flex_wrap != s2.flex_wrap {
        println!("flex_wrap: {:?} \n!=\n {:?}", s1.flex_wrap, s2.flex_wrap);
        return false;
    }
    if s1.flex_basis != s2.flex_basis {
        println!("flex_basis: {:?} \n!=\n {:?}", s1.flex_basis, s2.flex_basis);
        return false;
    }
    if s1.flex_grow != s2.flex_grow {
        println!("flex_grow: {:?} \n!=\n {:?}", s1.flex_grow, s2.flex_grow);
        return false;
    }
    if s1.flex_shrink != s2.flex_shrink {
        println!(
            "flex_shrink: {:?} \n!=\n {:?}",
            s1.flex_shrink, s2.flex_shrink
        );
        return false;
    }
    if s1.grid_template_rows != s2.grid_template_rows {
        println!(
            "grid_template_rows: {:?} \n!=\n {:?}",
            s1.grid_template_rows, s2.grid_template_rows
        );
        return false;
    }
    if s1.grid_template_columns != s2.grid_template_columns {
        println!(
            "grid_template_columns: {:?} \n!=\n {:?}",
            s1.grid_template_columns, s2.grid_template_columns
        );
        return false;
    }
    if s1.grid_auto_rows != s2.grid_auto_rows {
        println!(
            "grid_auto_rows: {:?} \n!=\n {:?}",
            s1.grid_auto_rows, s2.grid_auto_rows
        );
        return false;
    }
    if s1.grid_auto_columns != s2.grid_auto_columns {
        println!(
            "grid_auto_columns: {:?} \n!=\n {:?}",
            s1.grid_auto_columns, s2.grid_auto_columns
        );
        return false;
    }
    if s1.grid_auto_flow != s2.grid_auto_flow {
        println!(
            "grid_auto_flow: {:?} \n!=\n {:?}",
            s1.grid_auto_flow, s2.grid_auto_flow
        );
        return false;
    }
    if s1.grid_template_areas != s2.grid_template_areas {
        println!(
            "grid_template_areas: {:?} \n!=\n {:?}",
            s1.grid_template_areas, s2.grid_template_areas
        );
        return false;
    }
    if s1.grid_template_column_names != s2.grid_template_column_names {
        println!(
            "grid_template_column_names: {:?} \n!=\n {:?}",
            s1.grid_template_column_names, s2.grid_template_column_names
        );
        return false;
    }
    if s1.grid_template_row_names != s2.grid_template_row_names {
        println!(
            "grid_template_row_names: {:?} \n!=\n {:?}",
            s1.grid_template_row_names, s2.grid_template_row_names
        );
        return false;
    }
    if s1.grid_row != s2.grid_row {
        println!("grid_row: {:?} \n!=\n {:?}", s1.grid_row, s2.grid_row);
        return false;
    }
    if s1.grid_column != s2.grid_column {
        println!(
            "grid_column: {:?} \n!=\n {:?}",
            s1.grid_column, s2.grid_column
        );
        return false;
    }
    true
}
