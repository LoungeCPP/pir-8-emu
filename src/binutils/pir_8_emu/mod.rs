mod vm;
mod execution_config;

pub use self::vm::Vm;
pub use self::execution_config::ExecutionConfig;

pub mod display;


/// Contents of the icon to use for the `pir-8-emu` window
///
/// Cropped and scaled down version of [this](https://twitter.com/nabijaczleweli/status/1154176483730100224)
pub static ICON: &[u8] = include_bytes!("../../../assets/pir-8-emu.ico");


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
