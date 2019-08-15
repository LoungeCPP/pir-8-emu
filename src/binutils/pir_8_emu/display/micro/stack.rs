//! μStack window handling


use bear_lib_terminal::terminal::{print_xy, put_xy, clear};
use bear_lib_terminal::geometry::Rect;
use self::super::super::COLUMN_WIDTH;


/// Prepare the "μstack" window at the specified coords
///
/// The window is `27x2`, laid out as follows:
///
/// ```plaintext
/// μstack
/// 0x00, 0xA1
/// ```
///
/// Where the amount of μstack elements is variable
pub fn write(x_start: usize, y_start: usize) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    print_xy(x_start, y_start, "μstack");
    print_xy(x_start, y_start + 1, "{empty}");
}

/// Update the "μstack" window
///
/// See [`write()`](fn.write.html) for more info
pub fn update(x_start: usize, y_start: usize, stack: &[u8]) {
    let x_start = x_start as i32;
    let y_start = y_start as i32;

    let disp_len = if stack.is_empty() {
        print_xy(x_start, y_start + 1, "{empty}");
        7
    } else {
        for (i, b) in stack.iter().enumerate() {
            let i = i as i32;

            if i != 0 {
                put_xy(x_start + i * 6 - 2, y_start + 1, ',');
            }

            print_xy(x_start + i * 6, y_start + 1, "0x");
            print_xy(x_start + i * 6 + 2, y_start + 1, &format!("{:02X}", b));
        }

        stack.len() as i32 * 6 - 2
    };

    clear(Some(Rect::from_values(x_start + disp_len, y_start + 1, COLUMN_WIDTH - disp_len, 1)));
}
