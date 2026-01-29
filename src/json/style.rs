use regex::Regex;
use serde::{Deserialize, Serialize};
use taffy::{prelude::*, Overflow, Point, TextAlign};

use crate::json::helper_convertion;
fn default_one() -> f32 {
    1.0
}
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
    pub grid_row: (String, String),
    #[serde(default)]
    pub grid_column: (String, String),
}

fn to_dimension(s: &str) -> Dimension {
    let s = s.trim();

    if let Some(percent_str) = s.strip_suffix('%') {
        if let Ok(value) = percent_str.parse::<f32>() {
            return percent(value / 100.0);
        }
    }

    if let Ok(value) = s.parse::<f32>() {
        return length(value);
    }
    auto()
}

fn to_size(r: &str) -> Size<Dimension> {
    let re = Regex::new(r"(?<w>[^ ]+) (?<h>[^ ]+)").unwrap();
    if let Some(caps) = re.captures(&r) {
        // let w = caps["w"].to_string();
        // let h = caps["h"].to_string();
        Size {
            width: to_dimension(&caps["w"]),
            height: to_dimension(&caps["h"]),
        }
    } else {
        Size::auto()
    }
}

fn to_size_lp(r: &str) -> Size<LengthPercentage> {
    let re = Regex::new(r"(?<w>[^ ]+) (?<h>[^ ]+)").unwrap();
    if let Some(caps) = re.captures(&r) {
        Size {
            width: helper_convertion::to_length_percent(&caps["w"]),
            height: helper_convertion::to_length_percent(&caps["h"]),
        }
    } else {
        Size {
            width: LengthPercentage::ZERO,
            height: LengthPercentage::ZERO,
        }
    }
}

fn to_grid_template_components(s: Vec<String>) -> Vec<GridTemplateComponent<String>> {
    fn to_grid_template_component(s: String) -> GridTemplateComponent<String> {
        let s = s.trim();

        if let Some(percent_str) = s.strip_suffix('%') {
            if let Ok(value) = percent_str.parse::<f32>() {
                return percent(value / 100.0);
            }
        }

        if let Ok(value) = s.parse::<f32>() {
            return length(value);
        }
        auto()
    }
    s.into_iter().map(to_grid_template_component).collect()
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
            inset: helper_convertion::to_rect(&s.inset, Rect::auto()),

            size: to_size(&s.size),
            min_size: to_size(&s.min_size),
            max_size: to_size(&s.max_size),
            aspect_ratio: s.aspect_ratio,

            margin: helper_convertion::to_rect(&s.margin, Rect::zero()),
            padding: helper_convertion::to_rect_lp(&s.padding),
            border: helper_convertion::to_rect_lp(&s.border),

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
            grid_template_rows: to_grid_template_components(s.grid_template_rows),
            grid_template_columns: to_grid_template_components(s.grid_template_columns),
            grid_auto_flow: s.grid_auto_flow,
            grid_template_column_names: s.grid_template_column_names,
            grid_template_row_names: s.grid_template_row_names,
            // TODO
            // grid_auto_rows: to_grid_template_components(s.grid_template_rows),
            // grid_auto_columns: to_grid_template_components(s.grid_template_rows),
            // grid_template_areas: to_grid_template_components(s.grid_template_areas),
            // grid_row: (s.grid_row.0, s.grid_row.1),
            // grid_column: (s.grid_column.0, s.grid_column.1),
            ..Default::default()
        }
    }
}
