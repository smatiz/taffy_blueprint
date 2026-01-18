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
    } else if s == "" {
        length(0.0)
    } else {
        auto()
    }
}

pub fn to_length_percent<T>(s: String) -> T
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

pub fn to_rect<T>(r: Rect<String>) -> Rect<T>
where
    T: FromPercent + FromLength + TaffyAuto,
{
    Rect {
        left: to_length_percent_auto(&r.left),
        right: to_length_percent_auto(&r.right),
        top: to_length_percent_auto(&r.top),
        bottom: to_length_percent_auto(&r.bottom),
    }
}
pub fn to_rect_lp<T>(r: Rect<String>) -> Rect<T>
where
    T: FromPercent + FromLength,
{
    Rect {
        left: to_length_percent(r.left),
        right: to_length_percent(r.right),
        top: to_length_percent(r.top),
        bottom: to_length_percent(r.bottom),
    }
}
