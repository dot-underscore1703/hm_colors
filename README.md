# hm_colors
A simple Rust crate for converting hackmud color-code characters into ANSI color codes.

## Usage
```
use hm_colors;

let hackmud_color_character: char = 'D'; // This is the code for bright red.

let ansi_color_code: String = hm_colors::to_ansii(hackmud_color_character);
```
