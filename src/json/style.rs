use crate::json::helper_convertion::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use taffy::{prelude::*, Overflow, Point, TextAlign};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct StyleJson {
    #[serde(default)]
    pub display: Display,
    #[serde(default)]
    pub item_is_table: bool,
    #[serde(default)]
    pub item_is_replaced: bool,
    #[serde(default)]
    pub box_sizing: BoxSizing,
    #[serde(default)]
    pub overflow: Point<Overflow>,
    #[serde(default)]
    pub scrollbar_width: f32,
    #[serde(default)]
    pub position: Position,
    #[serde(default)]
    pub inset: String,

    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub min_size: String,
    #[serde(default)]
    pub max_size: String,
    #[serde(default)]
    pub aspect_ratio: Option<f32>,

    #[serde(default)]
    pub margin: String,
    #[serde(default)]
    pub padding: String,
    #[serde(default)]
    pub border: String,

    #[serde(default)]
    pub align_items: Option<AlignItems>,
    #[serde(default)]
    pub align_self: Option<AlignSelf>,
    #[serde(default)]
    pub justify_items: Option<AlignItems>,
    #[serde(default)]
    pub justify_self: Option<AlignSelf>,
    #[serde(default)]
    pub align_content: Option<AlignContent>,
    #[serde(default)]
    pub justify_content: Option<JustifyContent>,
    #[serde(default)]
    pub gap: String,
    #[serde(default)]
    pub text_align: TextAlign,

    #[serde(default)]
    pub flex_direction: FlexDirection,
    #[serde(default)]
    pub flex_wrap: FlexWrap,
    #[serde(default)]
    pub flex_basis: String,
    #[serde(default)]
    pub flex_grow: f32,
    #[serde(default = "default_one")]
    pub flex_shrink: f32,
    #[serde(default)]
    pub grid_template_rows: Vec<String>,
    #[serde(default)]
    pub grid_template_columns: Vec<String>,
    #[serde(default)]
    pub grid_auto_rows: Vec<String>,
    #[serde(default)]
    pub grid_auto_columns: Vec<String>,
    #[serde(default)]
    pub grid_auto_flow: GridAutoFlow,
    #[serde(default)]
    pub grid_template_areas: Vec<String>,
    #[serde(default)]
    pub grid_template_column_names: Vec<Vec<String>>,
    #[serde(default)]
    pub grid_template_row_names: Vec<Vec<String>>,
    #[serde(default)]
    pub grid_row: [String; 2],
    #[serde(default)]
    pub grid_column: [String; 2],
}

impl From<StyleJson> for Style {
    fn from(s: StyleJson) -> Self {
        Style {
            display: s.display,
            item_is_table: s.item_is_table,
            item_is_replaced: s.item_is_replaced,
            box_sizing: s.box_sizing,
            overflow: s.overflow,
            scrollbar_width: s.scrollbar_width,
            position: s.position,
            inset: to_rect(&s.inset, Rect::auto()),

            size: to_size(&s.size),
            min_size: to_size(&s.min_size),
            max_size: to_size(&s.max_size),
            aspect_ratio: s.aspect_ratio,

            margin: to_rect(&s.margin, Rect::zero()),
            padding: to_rect_lp(&s.padding),
            border: to_rect_lp(&s.border),

            align_items: s.align_items,
            align_self: s.align_self,
            justify_items: s.justify_items,
            justify_self: s.justify_self,
            align_content: s.align_content,
            justify_content: s.justify_content,
            gap: to_size_lp(&s.gap),
            text_align: s.text_align,

            flex_direction: s.flex_direction,
            flex_wrap: s.flex_wrap,
            flex_basis: to_dimension(&s.flex_basis),
            flex_grow: s.flex_grow,
            flex_shrink: s.flex_shrink,
            grid_template_rows: s
                .grid_template_rows
                .into_iter()
                .map(to_grid_template_component)
                .collect(),
            grid_template_columns: s
                .grid_template_columns
                .into_iter()
                .map(to_grid_template_component)
                .collect(),
            grid_auto_flow: s.grid_auto_flow,
            grid_template_column_names: s.grid_template_column_names,
            grid_template_row_names: s.grid_template_row_names,
            grid_auto_rows: s.grid_auto_rows.into_iter().map(to_min_max).collect(),
            grid_auto_columns: s.grid_auto_columns.into_iter().map(to_min_max).collect(),

            grid_row: Line {
                start: to_grid_placement(&s.grid_row[0]),
                end: to_grid_placement(&s.grid_row[1]),
            },
            grid_column: Line {
                start: to_grid_placement(&s.grid_column[0]),
                end: to_grid_placement(&s.grid_column[1]),
            },
            //   to_line(&s.grid_row),
            // grid_column: to_line(&s.grid_column),
            grid_template_areas: s
                .grid_template_areas
                .into_iter()
                .map(to_grid_template_areas)
                .collect(),
            dummy: PhantomData::default(),
        }
    }
}
