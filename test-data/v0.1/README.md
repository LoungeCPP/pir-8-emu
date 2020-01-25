# Test data as of v0.1.0 and v0.1.1

These manual derivations (.diz), assemblies (.p8a), and binaries (.p8b) construe the first code ever written for the pir-8.

Well, before the ISA got changed, anyway – v0.1.0 and v0.1.1 target
  [ISA 02c8070](https://github.com/thecoshman/pir-8/blob/02c807010162df431a2b948585a0a3f77cfba6d0/ISA.md),
  so the binaries from the current version are incompatible with the emulators of back then, and vice versa.

With that being said, this are the first (old-)pir-8 code, in chronological order:
1. `xor-swap`, which performs an xor-swap of registers `A` and `B`;
   note how this doesn't have an accompanying assembly, for a simple reason: the assembler hadn't existed at the time of writing
2. `xor-swap-with-loads` does a similar thing, except that it loads them first (`A` from memory and `B` from an immediate);
   this also didn't originally have an assembly, but it was added as part of testing the assembler
3. `gcd`, a functioning Greatest Common Divisor algorithm, using the first version of assembler directives
4. `copy-short-literal-to-port`, which essentially implemented a `strcpy()` to a destination port, for use in combination with the `message` example handler;
   however, due to the limitations of the ISA at the time, it needed to self-modify (see the `load-byte` label),
   and couldn't do more than the first 255 bytes.
   Well I guess the method *could* be, theoretically, extended to do it, but nobody implemented it, and overwriting two bytes instead of just one,
   without and add-with-carry was too much of a pain in the arse, even for me;
   still though, a functional proof of concept!

Notable changes between that ISA and [ISA 5f282f5](https://github.com/thecoshman/pir-8/blob/5f282f5e86cfc4add8818a201092c0e75be1c4cd/ISA.md), used in v1.0.0:
  * the `MADR` (Modify ADR) instruction, and its application for `LOAD IND`, `SAVE`, and the `JUMP` family;
    programs now need to `ADR` from a register pair, as those instructions no longer read the address from the data following them:
    ```asm
    LOAD IND A
    0x0110
    ```
    became
    ```asm
    LOAD IMM A
    0x01
    LOAD IMM B
    0x10
    MADR WRITE A&B
    LOAD IND A      ; With B clobbered
    ```
  * `ADR` is no longer just a buffer register showing the latest accessed location, and all three 16-bit registers (`PC`, `SP`, `ADR`) can now drive the bus.
  * `ALU NOT` got moved from `0010` to `0111`.
  * `ALU ADDC` and `ALU SUBC` were added, implementing add-with-carry and subtract-with-carry, respectively.
    Combined with the `[M]ADR` change, this means, for example, that an arbitrarily-long string can be copied to a port,
     without resorting to what can only be described as hacks –
     see the `copy-any-length-literal-to-port` example in v1.0.0, compare with `copy-short-literal-to-port` from here.

