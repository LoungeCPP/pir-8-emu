use dlopen::raw::Library as NativeLibrary;
use dlopen::Error as DlOpenError;
use std::ffi::OsStr;
use libc::c_void;



/// A [`PortHandler`](../../vm/trait.PortHandler.html) using the native API defined in
/// [`RawNativePortHandler`](struct.RawNativePortHandler.html)
#[derive(Debug)]
pub struct NativePortHandler {
    lib: NativeLibrary,
    raw: RawNativePortHandler,
    state: Option<*mut c_void>,
}

impl NativePortHandler {
    /// Load the handler from the DLL at the specified path
    ///
    /// If successful, the returned handler is ready to be used in [`Ports`](../../vm/struct.Ports.html)
    pub fn load_from_dll<P: AsRef<OsStr>>(path: &P) -> Result<NativePortHandler, DlOpenError> {
        NativePortHandler::load_from_dll_impl(NativeLibrary::open(path)?)
    }

    fn load_from_dll_impl(lib: NativeLibrary) -> Result<NativePortHandler, DlOpenError> {
        let raw = RawNativePortHandler {
            port_count: unsafe { lib.symbol_cstr(RawNativePortHandler::PORT_COUNT_NAME.as_cstr()) }?,
            init: unsafe { lib.symbol_cstr(RawNativePortHandler::INIT_NAME.as_cstr()) }?,
            uninit: unsafe { lib.symbol_cstr(RawNativePortHandler::UNINIT_NAME.as_cstr()) }?,
            handle_read: unsafe { lib.symbol_cstr(RawNativePortHandler::HANDLE_READ_NAME.as_cstr()) }?,
            handle_write: unsafe { lib.symbol_cstr(RawNativePortHandler::HANDLE_WRITE_NAME.as_cstr()) }?,
        };

        Ok(NativePortHandler {
            lib: lib,
            raw: raw,
            state: None,
        })
    }
}


/// Raw C funxion pointers into a loaded DLL
///
/// The raw in-/exported names are prefixed with `pir_8_emu_`
///
/// Consult [`PortHandler`](../../vm/trait.PortHandler.html) for timings and invariants
///
/// These correspond to the following C declaration set:
///
/// ```c
/// unsigned char pir_8_emu_port_count();
///
/// void * pir_8_emu_init(const unsigned char * ports, unsigned char ports_len);
///
/// void pir_8_emu_uninit(void * state);
///
/// unsigned char pir_8_emu_handle_read(void * state, unsigned char port);
///
/// void pir_8_emu_handle_write(void * state, unsigned char port, unsigned char byte);
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawNativePortHandler {
    /// Get the amount of ports this handler handles
    ///
    /// Returning `0` from this funxion will panic
    pub port_count: extern "C" fn() -> u8,

    /// Get the handler-allocated state corresponding to the specified ports set
    ///
    /// It *is* valid to return `nullptr` from this funxion
    ///
    /// The `ports_len` argument will always be equal to the last return value of `port_count()`
    pub init: extern "C" fn(ports: *const u8, ports_len: u8) -> *mut c_void,

    /// Release all resources associated with the specified state
    ///
    /// The `state` argument will always be equal to the one returned from `init()`
    pub uninit: extern "C" fn(state: *mut c_void),

    /// Handle the program reading from one of the handled ports
    ///
    /// The `port` argument will always have been contained within the set passed to `init()`
    ///
    /// The `state` argument will always be equal to the one returned from `init()`
    pub handle_read: extern "C" fn(state: *mut c_void, port: u8) -> u8,

    /// Handle the program writing to one of the handled ports
    ///
    /// The `port` argument will always have been contained within the set passed to `init()`
    ///
    /// The `state` argument will always be equal to the one returned from `init()`
    pub handle_write: extern "C" fn(state: *mut c_void, port: u8, byte: u8),
}

impl RawNativePortHandler {
    const_cstr! {
        pub PORT_COUNT_NAME   = "pir_8_emu_port_count";
        pub INIT_NAME         = "pir_8_emu_init";
        pub UNINIT_NAME       = "pir_8_emu_uninit";
        pub HANDLE_READ_NAME  = "pir_8_emu_handle_read";
        pub HANDLE_WRITE_NAME = "pir_8_emu_handle_write";
    }
}
