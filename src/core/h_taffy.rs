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
