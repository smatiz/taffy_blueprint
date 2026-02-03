use macroquad::prelude::*;

#[derive(Clone)]
pub struct TextDrawer {
    font: Option<Font>,
    font_size: u16,
    font_scale: f32,
}

impl TextDrawer {
    pub async fn new(font_size: u16) -> Self {
        let font = None; // load_ttf_font("assets/RobotoMono-Regular.ttf").await.ok();
        Self {
            font,
            font_size,
            font_scale: 1.0,
        }
    }

    pub fn measure(&self, text: &str) -> TextDimensions {
        measure_text(text, self.font.as_ref(), self.font_size, self.font_scale)
    }

    pub fn draw_rect(&self, text: &str, rect_screen: &Rect, color: Color) {
        let flip_y = false;
        draw_text_ex(
            text,
            rect_screen.x,
            rect_screen.y,
            TextParams {
                font: self.font.as_ref(),
                font_size: self.font_size,
                font_scale: self.font_scale * if flip_y { -1.0 } else { 1.0 },
                color,
                ..Default::default()
            },
        );
    }

    pub fn draw(&self, text: &str, x: f32, y: f32, color: Color) {
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: self.font.as_ref(),
                font_size: self.font_size,
                font_scale: self.font_scale,
                color,
                ..Default::default()
            },
        );
    }

    pub fn draw_exact(&self, text: &str, x: f32, y: f32, color: Color) {
        let m = self.measure(&text);
        self.draw(text, x, y + m.offset_y, color);
    }

    pub fn draw_centered_stretched(&self, text: &str, rect: &Rect, color: Color) {
        let measure_old = self.measure(text);
        let zoom = rect.h / measure_old.height;
        let new_font_size = ((self.font_size as f32) * zoom) as u16;
        let measure = measure_text(text, self.font.as_ref(), new_font_size, self.font_scale);

        let x = rect.x + (rect.w - measure.width) * 0.5;
        let y = rect.y + (rect.h - measure.height) * 0.5 + measure.offset_y;

        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: self.font.as_ref(),
                font_size: new_font_size,
                font_scale: self.font_scale,
                color,
                ..Default::default()
            },
        );
    }
    pub fn draw_centered(&self, text: &str, rect: &Rect, color: Color) {
        let m = self.measure(text);
        let x = rect.x + (rect.w - m.width) * 0.5;
        let y = rect.y + (rect.h - m.height) * 0.5 + m.offset_y;
        self.draw(text, x, y, color);
    }
}

// pub enum TextLayout {
//     VerticalCentered,
//     HorizontalCentered,
//     Centered,
//     Free,
// }

// #[derive(Default, Clone)]
// pub struct Text {
//     rect: Rect,
//     offset_y: f32,
//     pub text: String,
//     pub color: Color,
// }

// impl Text {
//     pub fn new() -> Self {
//         Self {
//             color: BLACK,
//             ..Default::default()
//         }
//     }
//     pub fn draw(&self, text_drawer: &TextDrawer) {
//         text_drawer.draw(
//             &self.text,
//             self.rect.x,
//             self.rect.y + self.offset_y,
//             self.color,
//         );
//     }
//     pub fn update(
//         &mut self,
//         text_drawer: &TextDrawer,
//         x: f32,
//         y: f32,
//         horiz: HorizontalAlignment,
//         vert: VerticalAlignment,
//     ) {
//         let m = text_drawer.measure(&self.text);
//         self.rect = h_layout::get_rect(
//             &Rect {
//                 x,
//                 y,
//                 w: m.width,
//                 h: m.height,
//             },
//             horiz,
//             vert,
//         );
//         self.offset_y = m.offset_y;
//     }

//     pub fn rect(&self) -> &Rect {
//         &self.rect
//     }
// }
// #[derive(Clone, Debug)]
// pub struct TextXXX {
//     measure: TextDimensions,
//     text: String,
// }
// impl TextXXX {
//     pub fn new(text: String) -> Self {
//         Self {
//             measure: TextDimensions::default(),
//             text,
//         }
//     }
//     pub fn update(&mut self, text_drawer: &TextDrawer) {
//         self.measure = text_drawer.measure(&self.text);
//     }

//     pub fn measure(&self) -> &TextDimensions {
//         &self.measure
//     }
// }
