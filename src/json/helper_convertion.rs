use once_cell::sync::Lazy;
use regex::Regex;
use taffy::{prelude::*, GridTemplateArea, MinMax};

pub(crate) static RE_2: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<w>[^ ]+) (?<h>[^ ]+)").unwrap());
static RE_4: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?<left>[^ ]+) (?<top>[^ ]+) (?<right>[^ ]+) (?<bottom>[^ ]+)").unwrap()
});
pub(crate) static RE_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"line ((?<name>[^ ]+) )?(?<v>[^ ]+)").unwrap());
pub(crate) static RE_SPAN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"span ((?<name>[^ ]+) )?(?<v>[^ ]+)").unwrap());

pub(crate) static RE_GRID_AREA: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?<name>[^ ]+) (?<row_start>[^ ]+) (?<row_end>[^ ]+) (?<column_start>[^ ]+) (?<column_end>[^ ]+)")
        .unwrap()
});
pub fn default_one() -> f32 {
    1.0
}

pub fn to_dimension(s: &str) -> Dimension {
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

fn min_tsf(s: &str) -> MinTrackSizingFunction {
    if s == "min" {
        return min_content();
    }
    if s == "max" {
        return max_content();
    }
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
fn max_tsf(s: &str) -> MaxTrackSizingFunction {
    if s == "min" {
        return min_content();
    }
    if s == "max" {
        return max_content();
    }
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

pub fn to_min_max(s: String) -> MinMax<MinTrackSizingFunction, MaxTrackSizingFunction> {
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

pub fn to_size(r: &str) -> Size<Dimension> {
    if let Some(caps) = RE_2.captures(&r) {
        Size {
            width: to_dimension(&caps["w"]),
            height: to_dimension(&caps["h"]),
        }
    } else {
        Size::auto()
    }
}

pub fn to_size_lp(r: &str) -> Size<LengthPercentage> {
    if let Some(caps) = RE_2.captures(&r) {
        Size {
            width: to_length_percent(&caps["w"]),
            height: to_length_percent(&caps["h"]),
        }
    } else {
        Size {
            width: LengthPercentage::ZERO,
            height: LengthPercentage::ZERO,
        }
    }
}
pub fn to_grid_template_component(s: String) -> GridTemplateComponent<String> {
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

pub fn try_percent<T>(s: &str) -> Option<T>
where
    T: FromPercent,
{
    s.strip_suffix('%')
        .and_then(|percent_str| percent_str.parse::<f32>().ok())
        .map(|value| percent(value / 100.0))
}
pub fn to_length_percent_auto<T>(s: &str) -> T
where
    T: FromPercent + FromLength + TaffyAuto,
{
    let s = s.trim();
    if let Some(value) = try_percent(s) {
        value
    } else if let Ok(value) = s.parse::<f32>() {
        length(value)
    } else if s.is_empty() {
        length(0.0)
    } else {
        auto()
    }
}

pub fn to_grid_placement(s: &str) -> GridPlacement {
    let s = s.trim();
    if s == "*" {
        GridPlacement::Auto
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

pub fn to_grid_template_areas(s: String) -> GridTemplateArea<String> {
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

pub fn to_length_percent<T>(s: &str) -> T
where
    T: FromPercent + FromLength,
{
    let s = s.trim();
    if let Some(value) = try_percent(s) {
        value
    } else {
        s.parse::<f32>()
            .map(|value| length(value))
            .unwrap_or(length(0.0))
    }
}
pub fn to_rect(r: &str, default: Rect<LengthPercentageAuto>) -> Rect<LengthPercentageAuto> {
    if let Some(caps) = RE_4.captures(r) {
        Rect {
            left: to_length_percent_auto(&caps["left"]),
            right: to_length_percent_auto(&caps["right"]),
            top: to_length_percent_auto(&caps["top"]),
            bottom: to_length_percent_auto(&caps["bottom"]),
        }
    } else {
        default
    }
}

pub fn to_rect_lp(r: &str) -> Rect<LengthPercentage> {
    if let Some(caps) = RE_4.captures(&r) {
        Rect {
            left: to_length_percent(&caps["left"]),
            right: to_length_percent(&caps["right"]),
            top: to_length_percent(&caps["top"]),
            bottom: to_length_percent(&caps["bottom"]),
        }
    } else {
        Rect::zero()
    }
}
