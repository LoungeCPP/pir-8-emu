//! `pir-8-emu`'s output, config, and execution handling


use std::time::Duration;

mod vm;
mod execution_config;
mod native_port_handler;

pub use self::vm::Vm;
pub use self::execution_config::ExecutionConfig;
pub use self::native_port_handler::{RawNativePortHandler, NativePortHandler};

pub mod display;


/// Contents of the icon to use for the `pir-8-emu` window
///
/// Cropped and scaled down version of [this](https://lfs.nabijaczleweli.xyz/0017-twitter-export#1154176483730100224)
pub static ICON: &[u8] = include_bytes!("../../../assets/pir-8-emu.ico");

/// Contents of the help text to display after pressing F1 in the `pir-8-emu` window, paginated by Form Feeds
pub static HELP_TEXT: &str = include_str!("../../../assets/pir-8-emu.hlp");

lazy_static! {
	/// Individual pages of the help text to display after pressing F1 in the `pir-8-emu` window, paginated by Form Feeds
	pub static ref HELP_TEXT_PAGES: Vec<&'static str> = HELP_TEXT.split('\x0C').map(str::trim).collect();
}

/// When waiting, check at least this often for new input
pub static MAX_UI_DELAY: Duration = Duration::from_millis(25);


/// Execute the contained funxion when this object is dropped
///
/// # Examples
///
/// ```
/// # use pir_8_emu::binutils::pir_8_emu::QuickscopeWrapper;
/// let mut hewwo = true;
/// {
/// #  let h = &hewwo as *const bool;
///    let _hewwo_destructor = QuickscopeWrapper(Some(|| hewwo = false));
/// #  assert!(unsafe { *h });
/// #  /*
///    assert!(hewwo);
/// #  */
/// }
/// assert!(!hewwo);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuickscopeWrapper<F: FnOnce()>(pub Option<F>);

impl<F: FnOnce()> Drop for QuickscopeWrapper<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}
