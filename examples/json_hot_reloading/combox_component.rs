use macroquad::prelude::*;

use crate::update_result::UpdateResult;
struct Item {
    pub text: String,
    pub text_position: Vec2,
    pub rect: Rect,
}
pub struct ComboBox {
    pub selected: Option<usize>,
    pub open: bool,
    items: Vec<Item>,
    text_position: Vec2,
    component_rect: Option<Rect>,
    font_size: f32,
}

const THICKNESS: f32 = 2.0;
const MARGIN: f32 = 2.0;
const CHOOSE: &str = "choose..";
impl ComboBox {
    pub fn new() -> Self {
        Self {
            selected: None,
            open: false,
            items: vec![],
            component_rect: None,
            font_size: 0.0,
            text_position: Vec2::ZERO,
        }
    }
    pub fn update(
        &mut self,
        items: Vec<String>,
        font_size: u16,
        x: f32,
        y: f32,
    ) -> UpdateResult<String> {
        self.font_size = font_size as f32;
        let text_r = measure_text(CHOOSE, None, font_size, 1.0);
        let mut w: f32 = text_r.width;
        let mut h: f32 = text_r.height;

        for item in items.iter() {
            let text_r = measure_text(&item, None, font_size, 1.0);
            w = w.max(text_r.width);
            h = h.max(text_r.height);
        }

        let component_rect = Rect {
            x: x,
            y: y,
            w: w + 2.0 * MARGIN,
            h: h + 2.0 * MARGIN,
        };
        self.text_position = vec2(
            x + MARGIN + (component_rect.w - text_r.width) * 0.5,
            y + MARGIN + text_r.offset_y,
        );
        self.component_rect = Some(component_rect);
        let (mx, my) = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left) && component_rect.contains(vec2(mx, my)) {
            self.open = !self.open;
        }
        self.items = items
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                let r = measure_text(&name, None, font_size, 1.0);
                Item {
                    text: name,
                    text_position: vec2(
                        x + MARGIN + (component_rect.w - r.width) * 0.5,
                        y + MARGIN + (1 + i) as f32 * component_rect.h + r.offset_y,
                    ),
                    rect: component_rect.offset(vec2(0.0, (1 + i) as f32 * component_rect.h)),
                }
            })
            .collect();

        if self.open {
            if is_mouse_button_pressed(MouseButton::Left) {
                for (i, item) in self.items.iter().enumerate() {
                    if item.rect.contains(vec2(mx, my)) {
                        self.selected = Some(i);
                        self.open = false;
                        return UpdateResult::End(self.items[i].text.to_string());
                    }
                }
            }
        }
        UpdateResult::Continue
    }

    pub fn draw(&self) {
        if let Some(component_rect) = self.component_rect {
            draw_rectangle_lines(
                component_rect.x,
                component_rect.y,
                component_rect.w,
                component_rect.h,
                THICKNESS,
                BLACK,
            );
            let text = if let Some(selected) = self.selected {
                &self.items[selected].text
            } else {
                CHOOSE
            };
            draw_text(
                text,
                self.text_position.x,
                self.text_position.y,
                self.font_size,
                BLACK,
            );

            if self.open {
                for item in self.items.iter() {
                    draw_rectangle_lines(
                        item.rect.x,
                        item.rect.y,
                        item.rect.w,
                        item.rect.h,
                        THICKNESS,
                        BLACK,
                    );
                    draw_text(
                        &item.text,
                        item.text_position.x,
                        item.text_position.y,
                        self.font_size,
                        BLACK,
                    );
                }
            }
        }
    }
}
