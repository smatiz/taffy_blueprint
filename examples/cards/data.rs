use crate::components::{Character, Class};

pub const CHARACTERS: &[Character; 3] = &[
    Character {
        class: Class::Mage,
        agility: 2,
        intelligence: 3,
        mana: 6,
        strength: 1,
    },
    Character {
        class: Class::Warrior,
        agility: 0,
        intelligence: 1,
        mana: 0,
        strength: 6,
    },
    Character {
        class: Class::Rogue,
        agility: 5,
        intelligence: 4,
        mana: 0,
        strength: 2,
    },
];
