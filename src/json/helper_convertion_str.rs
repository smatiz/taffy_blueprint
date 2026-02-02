use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use taffy::{prelude::*, GridTemplateArea, MinMax};

const NAMESPACE: &str = "taffy::prelude::";
pub fn to_dimension_str(s: &str) -> String {
    
    let s = s.trim();

    if let Some(percent_str) = s.strip_suffix('%') {
        if let Ok(value) = percent_str.parse::<f32>() {
            return _percent(value);
        }
    }

    if let Ok(value) = s.parse::<f32>() {
        return _length(value);
    }
    return _auto();
}

fn max_tsf_str(s: &str) -> String {
    if s == "min" {
        return min_content();
    }
    if s == "max" {
        return max_content();
    }
    if let Some(percent_str) = s.strip_suffix('%') {
        if let Ok(value) = percent_str.parse::<f32>() {
            return _percent(value / 100.0);
        }
    }
    if let Ok(value) = s.parse::<f32>() {
        return _length(value);
    }
    _auto()
}

pub fn to_min_max_str(s: String) -> String {
    let s = s.trim();
    if let Some(caps) = RE_2.captures(&s) {
        MinMax {
            min: min_tsf(&caps["w"]),
            max: max_tsf(&caps["w"]),
        }
    } else {
        MinMax::AUTO
    }
}

pub fn to_size_str(r: &str) -> String {
    if let Some(caps) = RE_2.captures(&r) {
        Size {
            width: to_dimension(&caps["w"]),
            height: to_dimension(&caps["h"]),
        }
    } else {
        Size::auto()
    }
}

pub fn to_size_lp_str(r: &str) -> String {
    if let Some(caps) = RE_2.captures(&r) {
   format!( r#"
    {0}::Size {{
        width: {0}{1}({2}),
        height: {0}{1}({3}),
    }}
        "#,)
        Size {
            width: to_length_percent_str(&caps["w"]),
            height: to_length_percent_str(&caps["h"]),
        }
    } else {
        format!( r#"
    {0}::Size {{
        width: {0}::LengthPercentage::ZERO,
        height: {0}::LengthPercentage::ZERO,
    }}
        "#,NAMESPACE)
        
    }
}
pub fn to_grid_template_component_str(s: String) -> String {
    let s = s.trim();

    if let Some(percent_str) = s.strip_suffix('%') {
        if let Ok(value) = percent_str.parse::<f32>() {
            return _percent(value / 100.0);
        }
    }

    if let Ok(value) = s.parse::<f32>() {
        return _length(value);
    }
    _auto()
}

pub fn try_percent_str(s: &str) -> Option<String> {
    s.strip_suffix('%')
        .and_then(|percent_str| percent_str.parse::<f32>().ok())
        .map(|value| _percent(value))
}
pub fn to_length_percent_auto_str<T>(s: &str) -> String
where
    T: FromPercent + FromLength + TaffyAuto,
{
    let s = s.trim();
    if let Some(value) = try_percent_str(s) {
        value
    } else if let Ok(value) = s.parse::<f32>() {
        _length(value)
    } else if s.is_empty() {
        _length(0.0)
    } else {
        _auto()
    }
}

pub fn to_grid_placement_str(s: &str) -> String {
    let s = s.trim();
    if s == "*" {
        "GridPlacement::Auto".into()
    } else if let Some(caps) = RE_LINE.captures(s) {
        if caps.name("name").is_none() {
            GridPlacement::Line(caps["v"].parse::<i16>().unwrap().into())
        } else {
            GridPlacement::NamedLine(caps["name"].to_string(), caps["v"].parse::<i16>().unwrap())
        }
    } else if let Some(caps) = RE_SPAN.captures(s) {
        if caps.name("name").is_none() {
            GridPlacement::Span(s.parse::<u16>().unwrap())
        } else {
            GridPlacement::NamedSpan(caps["name"].to_string(), caps["v"].parse::<u16>().unwrap())
        }
    } else {
        GridPlacement::Auto
    }
}

pub fn to_grid_template_areas_str(s: String) -> String {
    if let Some(caps) = RE_GRID_AREA.captures(&s) {
        GridTemplateArea {
            name: caps["name"].into(),
            row_start: caps["row_start"].parse::<u16>().unwrap(),
            row_end: caps["row_end"].parse::<u16>().unwrap(),
            column_start: caps["column_start"].parse::<u16>().unwrap(),
            column_end: caps["column_end"].parse::<u16>().unwrap(),
        }
    } else {
        panic!("invalid grid area");
    }
}
pub fn to_length_percent_str(s: &str) -> String {
    let s = s.trim();
    if let Some(value) = try_percent_str(s) {
        value
    } else {
        s.parse::<f32>()
            .map(|value| _length(value))
            .unwrap_or(_length(0.0))
    }
}

fn to_rect_ok_str(f: &str, caps: Captures) -> String {
    format!(
        r#"
    {0}::Rect {{
        left: {0}{1}({2}),
        right: {0}{1}({3}),
        top: {0}{1}({4}),
        bottom: {0}{1}({5}),
    }}
        "#,
        NAMESPACE, f, &caps["left"], &caps["right"], &caps["top"], &caps["bottom"]
    )
}

pub fn to_rect_str(r: &str, default: &str) -> String {
    if let Some(caps) = RE_4.captures(r) {
        to_rect_ok_str("to_length_percent_auto", caps)
    } else {
        default.into()
    }
}

pub fn to_rect_lp_str(r: &str) -> String {
    if let Some(caps) = RE_4.captures(&r) {
        to_rect_ok_str("to_length_percent", caps)
    } else {
        format!("{}Rect::zero()", NAMESPACE)
    }
}

fn _length(value: f32) -> String {
    format!("{}length({})", NAMESPACE, value)
}
fn _auto() -> String {
    format!("{}auto()", NAMESPACE)
}
fn _percent(value: f32) -> String {
    format!("{}percent({})", NAMESPACE, value / 100.0)
}
