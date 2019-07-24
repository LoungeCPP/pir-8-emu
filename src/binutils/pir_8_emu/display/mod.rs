use bear_lib_terminal::Color;

pub mod register;


pub fn colours_for_rw(read: bool, written: bool) -> (Color, Color) {
    let fg = if read {
        Color::from_rgb(0x00, 0x00, 0x00)
    } else {
        Color::from_rgb(0xFF, 0xFF, 0xFF)
    };

    let bg = Color::from_rgb(if written { 0xFF } else { 0x00 }, if read { 0xFF } else { 0x00 }, 0x00);

    (fg, bg)
}
