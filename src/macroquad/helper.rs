use macroquad::prelude::*;

pub fn string_to_color(s: &str) -> Option<Color> {
    let s = s.trim().to_lowercase();

    // 1. Nomi predefiniti
    match s.as_str() {
        "white" => return Some(WHITE),
        "black" => return Some(BLACK),
        "red" => return Some(RED),
        "green" => return Some(GREEN),
        "blue" => return Some(BLUE),
        "yellow" => return Some(YELLOW),
        "orange" => return Some(ORANGE),
        "pink" => return Some(PINK),
        "purple" => return Some(PURPLE),
        "gray" | "grey" => return Some(GRAY),
        _ => {}
    }

    // 2. Hex: #RRGGBB o #RRGGBBAA (con o senza #)
    let hex = if s.starts_with('#') { &s[1..] } else { &s };

    fn parse_hex_pair(pair: &str) -> Option<u8> {
        u8::from_str_radix(pair, 16).ok()
    }

    match hex.len() {
        6 => {
            let r = parse_hex_pair(&hex[0..2])?;
            let g = parse_hex_pair(&hex[2..4])?;
            let b = parse_hex_pair(&hex[4..6])?;
            Some(Color::from_rgba(r, g, b, 255))
        }
        8 => {
            let r = parse_hex_pair(&hex[0..2])?;
            let g = parse_hex_pair(&hex[2..4])?;
            let b = parse_hex_pair(&hex[4..6])?;
            let a = parse_hex_pair(&hex[6..8])?;
            Some(Color::from_rgba(r, g, b, a))
        }
        _ => None,
    }
}
