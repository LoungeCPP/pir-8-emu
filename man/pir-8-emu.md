pir-8-emu(1) -- Emulator of the pir-8
=====================================

## SYNOPSIS

`pir-8-emu` [CONFIG_DIR]

## DESCRIPTION

Implementation of the pir-8 ISA.

First, compile your source code with pir-8-as(1).

Alternatively, check the content of a binary with pir-8-disasm(1).

To emulate, open pir-8-emu(1) and press *Ctrl+O* to open a binary,
then *Space* to execute step-by-step. For more usage consult the
in-emulator help available after pressing *F1*, or detailed below.

Read values appear green, and written ones appear red.

## CONTROLS

    General key mappings:
      F1     – Show this help message
      Ctrl+O – Open a memory image and reset the emulation therewith
      Ctrl+C – Close the emulator

    Execution key mappings:
      Ctrl+B      – Add a breakpoint
      Ctrl+G      – Remove a breakpoint
      Escape      – Clear active breakpoint
      Ctrl+U      – Update a memory address
      Ctrl+J      – Read in a memory address, finish current μOps, and jump thereto
      Space       – Perform the highlighted μOp if execute full instructions is OFF,
                    otherwise execute the current instruction
      Shift+Space – Read in a frequency and press Space thereat
      Ctrl+Space  – Silently execute until input/end/breakpoint

    Port-related key mappings:
      Ctrl+R – Read a byte from a port
      Ctrl+W – Write a byte to a port
      Ctrl+I – Install a native port handler
      Ctrl+K – Uninstall a port handler

    Config key mappings:
      Ctrl+Shift+A – Toggle auto load next instruction
      Ctrl+Shift+F – Toggle execute full instructions
      Ctrl+Shift+R – Rename general-purpose registers

## PORT HANDLERS

The `pir-8-emu` library supports all sorts of handlers that can
take control of a `pir-8` port. However, as recompiling to
change a handler would be less than ideal for a real-time emulator,
this emulator allows installing native port handlers.

A native port handler is any dynamically-loaded library that provides
the following interface:

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

Also available as [`include/pir-8-emu/port_handler.h`](https://github.com/LoungeCPP/pir-8-emu/blob/master/include/pir-8-emu/port_handler.h) in the git repo.

For more information about native port handlers
visit the [`RawNativePortHandler`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/doc/pir_8_emu/binutils/pir_8_emu/struct.RawNativePortHandler.html) doc page.

For more information about port handlers in general
visit the [`Ports`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/doc/pir_8_emu/vm/struct.Ports.html) doc page

## OPTIONS

  CONFIG_DIR

    Directory containing configuration files

    Default: $HOME/.pir-8-emu/

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
