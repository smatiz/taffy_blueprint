use regex::Regex;
use taffy::prelude::*;
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
pub fn to_rect(r: &str) -> Rect<LengthPercentageAuto> {
    let re = Regex::new(r"(?<left>\w) (?<top>\w) (?<right>\w) (?<bottom>\w)").unwrap();

    if let Some(caps) = re.captures(&r) {
        Rect {
            left: to_length_percent_auto(&caps["left"]),
            right: to_length_percent_auto(&caps["right"]),
            top: to_length_percent_auto(&caps["top"]),
            bottom: to_length_percent_auto(&caps["bottom"]),
        }
    } else {
        Rect::auto()
    }
}

pub fn to_rect_lp(r: &str) -> Rect<LengthPercentage> {
    let re = Regex::new(r"(?<left>\w) (?<top>\w) (?<right>\w) (?<bottom>\w)").unwrap();
    if let Some(caps) = re.captures(&r) {
        Rect {
            left: to_length_percent(&caps["left"]),
            right: to_length_percent(&caps["right"]),
            top: to_length_percent(&caps["top"]),
            bottom: to_length_percent(&caps["bottom"]),
        }
    } else {
        Rect {
            left: LengthPercentage::ZERO,
            right: LengthPercentage::ZERO,
            top: LengthPercentage::ZERO,
            bottom: LengthPercentage::ZERO,
        }
    }
}
