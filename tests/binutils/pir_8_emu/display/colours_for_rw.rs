use pir_8_emu::binutils::pir_8_emu::display::colours_for_rw;
use bear_lib_terminal::Color;


#[test]
fn not_read_not_written() {
    assert_eq!(colours_for_rw(false, false),
               (Color::from_rgb(0xFF, 0xFF, 0xFF), Color::from_rgb(0x00, 0x00, 0x00)));
}

#[test]
fn not_read_yes_written() {
    assert_eq!(colours_for_rw(false, true),
               (Color::from_rgb(0xFF, 0xFF, 0xFF), Color::from_rgb(0xFF, 0x00, 0x00)));
}

#[test]
fn yes_read_not_written() {
    assert_eq!(colours_for_rw(true, false),
               (Color::from_rgb(0x00, 0x00, 0x00), Color::from_rgb(0x00, 0xFF, 0x00)));
}

#[test]
fn yes_read_yes_written() {
    assert_eq!(colours_for_rw(true, true),
               (Color::from_rgb(0x00, 0x00, 0x00), Color::from_rgb(0xFF, 0xFF, 0x00)));
}
