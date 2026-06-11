use std::collections::HashMap;
use once_cell::sync::Lazy;

fn hex_to_rgb(hex: &str) -> Option<String> {
    let hex = hex.trim_start_matches("#");
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(format!("\x1b[38;2;{r};{g};{b}m"))
}

pub static COLOR_MAP: Lazy<HashMap<char, &str>> = Lazy::new(|| {
    HashMap::from([
        ('0', "#9B9B9B"), ('C', "#9B9B9B"),
        ('1', "#FFFFFF"), ('A', "#FFFFFF"),
        ('2', "#1EFF00"), ('L', "#1EFF00"),
        ('3', "#0070DD"), ('P', "#0070DD"),
        ('4', "#B035EE"), ('T', "#B035EE"),

        ('D', "#FF0000"),
        ('E', "#FF8383"),
        ('G', "#F3AAA6"),
        ('H', "#FBC803"),
        ('I', "#FFD863"),
        ('J', "#FFF404"),
        ('K', "#F3F998"),
        ('M', "#B3FF9B"),
        ('N', "#00FFFF"),
        ('O', "#8FE6FF"),
        ('Q', "#A4E3FF"),
        ('R', "#0000FF"),
        ('S', "#7AB2F4"),
        ('U', "#E6C4FF"),
        ('V', "#FF00EC"),
        ('W', "#FF96E0"),
        ('X', "#FF0070"),
        ('Y', "#FF6A98"),
        ('Z', "#0C112B"),

        ('a', "#000000"),
        ('b', "#3F3F3F"),
        ('c', "#676767"),
        ('d', "#7D0000"),
        ('e', "#8E3434"),
        ('f', "#A34F00"),
        ('g', "#725437"),
        ('h', "#A88600"),
        ('i', "#B2934A"),
        ('j', "#939500"),
        ('k', "#495225"),
        ('l', "#299400"),
        ('m', "#23381B"),
        ('n', "#00535B"),
        ('o', "#324A4C"),
        ('p', "#0073A6"),
        ('q', "#385A6C"),
        ('r', "#010067"),
        ('s', "#507AA1"),
        ('t', "#601C81"),
        ('u', "#43314C"),
        ('v', "#8C0069"),
        ('w', "#973984"),
        ('x', "#880024"),
        ('y', "#762E4A"),
        ('z', "#101215"),

        ('5', "#FF8000"),
        ('6', "#FF8000"),
        ('7', "#FF8000"),
        ('8', "#FF8000"),
        ('9', "#FF8000"),
        ('F', "#FF8000"),
    ])
});

pub fn to_ansi(hm_colorcode: char) -> Option<String> {
    COLOR_MAP
        .get(&hm_colorcode)
        .and_then(|hex| hex_to_rgb(hex))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_color_uppercase() {
        let ansi = to_ansi('A').unwrap();
        assert_eq!(ansi, hex_to_rgb("#FFFFFF").unwrap());
    }

    #[test]
    fn test_known_color_lowercase() {
        let ansi = to_ansi('m').unwrap();
        assert_eq!(ansi, hex_to_rgb("#23381B").unwrap());
    }

    #[test]
    fn test_grouped_color() {
        let ansi = to_ansi('7').unwrap();
        assert_eq!(ansi, hex_to_rgb("#FF8000").unwrap());
    }

    #[test]
    fn test_invalid_code_returns_none() {
        assert!(to_ansi('?').is_none());
    }

    #[test]
    fn test_hex_to_rgb_valid() {
        let ansi = hex_to_rgb("#123456").unwrap();
        assert_eq!(ansi, "\x1b[38;2;18;52;86m");
    }

    #[test]
    fn test_hex_to_rgb_invalid_length() {
        assert!(hex_to_rgb("#123").is_none());
    }

    #[test]
    fn test_hex_to_rgb_invalid_chars() {
        assert!(hex_to_rgb("#GGGGGG").is_none());
    }
}