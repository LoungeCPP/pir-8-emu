// The MIT License (MIT)

// Copyright (c) 2019 Lounge<C++>

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


#ifdef __cplusplus
extern "C" {
#endif


//! Exporting these funxions will allow a DLL to be used as a native port handler in `pir-8-emu`
//!
//! For more information about native port handlers visit the
//! [`RawNativePortHandler`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/doc/pir_8_emu/binutils/pir_8_emu/struct.RawNativePortHandler.html) doc page.
//!
//! For more information about port handlers lifetimes et al. in general visit the
//! [`Ports`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/doc/pir_8_emu/vm/struct.Ports.html) doc page
//!
//! For an example implementation see [TBD]


/// Get the amount of ports this handler handles
///
/// Returning `0` from this funxion will panic the emulator
unsigned char pir_8_emu_port_count();

/// Initialise the handler state with the specified ports
///
/// The returned value will be passed in the following funxions as the `state` argument
void * pir_8_emu_init(const unsigned char * ports, unsigned char ports_len);

/// Release all resources associated with the specified state
void pir_8_emu_uninit(void * state);

/// Handle the program reading from one of the handled ports
unsigned char pir_8_emu_handle_read(void * state, unsigned char port);

/// Handle the program writing to one of the handled ports
void pir_8_emu_handle_write(void * state, unsigned char port, unsigned char byte);


#ifdef __cplusplus
}
#endif
