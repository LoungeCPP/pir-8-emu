//! An implementation of the [`pir-8` ISA](https://github.com/thecoshman/pir-8/blob/master/ISA.md).
//!
//! # The library
//!
//! [`pir-8-emu`](https://github.com/LoungeCPP/pir-8-emu) can be thought of as consisting of layers:
//!
//! The first layer is the [`isa`](isa/) module,
//! which contains a pure implementation of the [`pir-8` ISA](https://github.com/thecoshman/pir-8/blob/master/ISA.md),
//! and can be used on its own to parse/generate binaries at the instruction level.
//!
//! The second layer is the [`vm`](vm/) module,
//! which contains parts of VM memory and port handling.
//!
//! The third layer is the [`micro`](micro/) module,
//! which contains a full stack-based microcode implementation,
//! and can be used to fully emulate a `pir-8` machine (see example inside).
//!
//! The fourth layer is the various [`binutils`](binutils/),
//! which contain useful parts of the executables,
//! like [`AssemblerDirective`](binutils/pir_8_as/enum.AssemblerDirective.html) and
//!      [`OutputWithQueue`](binutils/pir_8_as/struct.OutputWithQueue.html),
//! or [`NativePortHandler`](binutils/pir_8_emu/struct.NativePortHandler.html).
//!
//! These utilities can be used to quickly and correctly build off existing solutions,
//! but may have some quirks or be less absolutely generic
//! (e.g. [`Vm`](binutils/pir_8_emu/struct.Vm.html) will allow you to integrate
//!  a fully (as-emulator) controllable and functional `pir-8` virtual machine in about 5 lines,
//!  but it needs to have the `INS` SP register be observed after each μOp (see example inside)).
//!
//! # The binaries
//!
//! The headers link to manpages with more detailed usage instructions:
//!
//! ## [`pir-8-as`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/man/pir-8-as.1.html)
//!
//! An assembler with an… idiosyncratic syntax:
//!
//! ```p8a
//! JUMP
//! :label load text
//!
//! :literal "*pounces on u* OwO what's whis?"
//! 0x00
//!
//! :label save text
//! LOAD IMM Y
//! 1
//!
//! LOAD IMM A
//! 0x00
//!
//! LOAD IMM X
//! 0x03
//!
//! :label save loop-head
//! SAVE X
//! :label load-offset load-byte 2
//!
//! JUMP
//! :label load load-byte
//!
//! :label save post-load
//! PORT OUT S
//! COMP S
//! JMPZ
//! :label load end
//!
//! ALU ADD
//! MOVE S X
//! JUMP
//! :label load loop-head
//!
//! :label save end
//! HALT
//!
//! :label save load-byte
//! LOAD IND S
//! 0x0000
//! JUMP
//! :label load post-load
//! ```
//!
//! If you'd rather use a more normal syntax, [CatPlusPlus](https://github.com/TheCatPlusPlus) has also made
//! a [`fasm`-based assembler](https://github.com/TheCatPlusPlus/pir8/tree/master/Assembler):
//!
//! ```asm
//! include 'pir8.finc'
//!
//! origin 0x0002
//!
//! 	load a, [0x0000]
//! 	load b, [0x0001]
//!
//! top:
//! 	move x, a
//! 	move y, b
//! 	sub
//!
//! 	jmpz exit
//!
//! 	move s, a
//! 	comp b
//!
//! 	jmpl lt
//!
//! 	sub
//! 	move a, s
//! 	jump top
//!
//! lt:
//! 	move y, a
//! 	move x, b
//! 	sub
//! 	move b, s
//! 	jump top
//!
//! exit:
//! 	move d, a
//! 	halt
//! ```
//!
//! ## [`pir-8-disasm`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/man/pir-8-disasm.1.html)
//!
//! A dissassembler with a [`ndisasm`](https://www.nasm.us)-based frontend:
//!
//! ```plaintext
//! $ pir-8-disasm -k 1,7 test-data/xor-swap-with-loads.p8b
//! 00000000   24   LOAD IND A
//! 00000002 0110 D 0x0110
//! 00000003   1D   LOAD IMM B
//! 00000004   69 D 0x69
//! 00000005   62   MOVE A X
//! 00000006   6B   MOVE B Y
//! 00000007   35   ALU XOR
//! 00000008      S skipping 0x07 bytes
//! 00000010   4C   MOVE S A
//! 00000011   FF   HALT
//! ```
//!
//! ## [`pir-8-emu`](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/man/pir-8-emu.1.html)
//!
//! The emulator in-of itself:
//!
//! ![Emulator screenshot](data:image/png;base64,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)
//!
//! # Example programs
//!
//! Apart from the two forthlaid above,
//! take a look at the [`test-data/`](https://github.com/LoungeCPP/pir-8-emu/tree/master/test-data) directory in the git repo,
//! which contains a mix of assembler programs (`.p8a`), program binaries (`.p8b`), and derivations/hand-assemblies (`.diz`).
//!
//! # Native handlers
//!
//! For more information,
//! consult the documentation on [`RawNativePortHandler`](binutils/pir_8_emu/struct.RawNativePortHandler.html).
//!
//! For examples, take a look at the [`handler-examples/`](https://github.com/LoungeCPP/pir-8-emu/tree/master/handler-examples)
//! directory in the git repo.
//! Running `make` at the root thereof *should* build them without much hassle,
//! if it doesn't, please [open an issue](https://github.com/LoungeCPP/pir-8-emu/issues).
//!
//! The
//! [`include/pir-8-emu/port_handler.h`](https://github.com/LoungeCPP/pir-8-emu/tree/master/include/pir-8-emu/port_handler.h)
//! file contains C declarations.
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD

extern crate bear_lib_terminal;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate lazy_static;
extern crate arraydeque;
#[macro_use]
extern crate const_cstr;
extern crate num_traits;
extern crate dlopen;
extern crate serde;
#[macro_use]
extern crate clap;
extern crate libc;
extern crate dirs;
extern crate toml;

mod rw;

pub mod vm;
pub mod isa;
pub mod util;
pub mod micro;
pub mod options;
pub mod binutils;

pub use self::rw::{ReadWriteMarker, ReadWritable};
