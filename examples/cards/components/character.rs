use macroquad::{
    color::BLACK,
    math::vec2,
    shapes::{draw_circle_lines, draw_line},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Class {
    Mage,
    Warrior,
    Rogue,
}

impl Class {
    pub fn name(&self) -> String {
        match self {
            Self::Mage => "Mage".to_string(),
            Self::Warrior => "Warrior".to_string(),
            Self::Rogue => "Rogue".to_string(),
        }
    }

    pub fn draw(&self, rect: &macroquad::math::Rect) {
        match self {
            Class::Mage => {
                let radius = rect.h * 0.33 * 0.5;
                let small_radius = rect.h * 0.33 * 0.25;
                draw_circle_lines(rect.x + rect.w * 0.5, rect.y + radius, radius, 1.0, BLACK);
                draw_circle_lines(
                    rect.x + rect.w * 0.5,
                    rect.y + radius,
                    small_radius,
                    1.0,
                    BLACK,
                );
                draw_line(
                    rect.x + rect.w * 0.5,
                    rect.y + radius * 2.0,
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h,
                    1.0,
                    BLACK,
                );
            }
            Class::Warrior => {
                draw_circle_lines(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.5,
                    rect.h * 0.5,
                    1.0,
                    BLACK,
                );
                draw_line(
                    rect.x + rect.w * 0.5,
                    rect.y,
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h,
                    1.0,
                    BLACK,
                );
                draw_line(
                    rect.x,
                    rect.y + rect.h * 0.5,
                    rect.x + rect.w,
                    rect.y + rect.h * 0.5,
                    1.0,
                    BLACK,
                );
            }
            Class::Rogue => {
                let d = 0.2;

                let ps = [
                    vec2(0.0, 0.5),
                    vec2(0.25, 0.5 - d),
                    vec2(0.75, 0.5 + d),
                    vec2(1.0, 0.5),
                    vec2(0.75, 0.5 - d),
                    vec2(0.25, 0.5 + d),
                ];
                for i in 0..ps.len() {
                    let j = if i == ps.len() - 1 { 0 } else { i + 1 };
                    draw_line(
                        rect.x + rect.w * ps[i].x,
                        rect.y + rect.h * ps[i].y,
                        rect.x + rect.w * ps[j].x,
                        rect.y + rect.h * ps[j].y,
                        1.0,
                        BLACK,
                    );
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Character {
    pub name: &'static str,
    pub class: Class,
    pub mana: u8,
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
}
