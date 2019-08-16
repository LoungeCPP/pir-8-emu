use libc::c_void;


/// Raw C function pointers into a loaded DLL
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
pub struct RawNativePortHandler {
    /// Get the amount of ports this handler handles
    ///
    /// Returning `0` from this function will panic
    pub port_count: extern "C" fn() -> u8,

    /// Get the handler-allocated state corresponding to the specified ports set
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


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NativePortHandler {}
