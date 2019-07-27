mod vm;

pub use self::vm::Vm;

pub mod display;


/// Contents of the icon to use for the `pir-8-emu` window
///
/// Cropped and scaled down version of [this](https://twitter.com/nabijaczleweli/status/1154176483730100224)
pub static ICON: &[u8] = include_bytes!("../../../assets/pir-8-emu.ico");
