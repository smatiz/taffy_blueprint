use macroquad::{
    color::BLACK,
    shapes::{draw_circle, draw_line},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Class {
    Mage,
    Warrior,
    Rogue,
}

impl Class {
    pub fn class_name(&self) -> String {
        match self {
            Self::Mage => "Mage".to_string(),
            Self::Warrior => "Warrior".to_string(),
            Self::Rogue => "Rogue".to_string(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::Mage => "John Spellweaverry".to_string(),
            Self::Warrior => "Arnold Blademasterry".to_string(),
            Self::Rogue => "Tom Tricksterry".to_string(),
        }
    }

    pub fn draw(&self, rect: &macroquad::math::Rect) {
        match self {
            Class::Mage => {
                draw_circle(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * (0.66 + 0.33 * 0.5),
                    rect.h * 0.33,
                    BLACK,
                );
                draw_circle(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * (0.66 + 0.33 * 0.5),
                    rect.h * 0.33 * 0.5,
                    BLACK,
                );
                draw_line(
                    rect.x + rect.w * 0.5,
                    rect.y,
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.66,
                    1.0,
                    BLACK,
                );
            }
            Class::Warrior => {
                draw_circle(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.5,
                    rect.h * 0.5,
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
                // TODO
                draw_line(
                    rect.x + rect.w * 0.5,
                    rect.y,
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h,
                    1.0,
                    BLACK,
                );
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Character {
    pub class: Class,
    pub mana: u8,
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
}
