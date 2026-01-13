use super::*;
use taffy::prelude::*;

pub fn style_dimension(width: f32, height: f32) -> Style {
    Style {
        size: Size {
            width: length(width),
            height: length(height),
        },
        ..Default::default()
    }
}

pub fn style_auto() -> Style {
    Style {
        size: Size {
            width: auto(),
            height: auto(),
        },
        ..Default::default()
    }
}

pub fn style_full() -> Style {
    Style {
        size: Size {
            width: percent(1.0),
            height: percent(1.0),
        },
        ..Default::default()
    }
}

pub trait TextSize {
    fn measure(&self, text:&str)-> (f32,f32);
}

pub fn get_text_style(td: impl TextSize, text: &str) -> Style {
    let r = td.measure(text);
    h_taffy::style_dimension(r.0, r.1)
}
pub fn margin(d: f32) -> taffy::Rect<LengthPercentageAuto> {
    taffy::Rect::<LengthPercentageAuto> {
        left: length(d),
        right: length(d),
        top: length(d),
        bottom: length(d),
    }
}
