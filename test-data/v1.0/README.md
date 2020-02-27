# Test data as of v1.0.0

These assemblies (.p8a) and binaries (.p8b) construe the test code as it appeared in v1.0.0,
  targetting [ISA 5f282f5](https://github.com/thecoshman/pir-8/blob/5f282f5e86cfc4add8818a201092c0e75be1c4cd/ISA.md);
  note, that the binaries from the current version are incompatible with the emulators of back then, and vice versa.

These are:
1. `copy-any-length-literal-to-port`, an evolution of `copy-short-literal-to-port` from v0.1, which can now `strcpy()` a literal of any length to a port;
   the code, however, is kinda clumsy, as that ISA removed all multi-byte loads.

Notable changes between that ISA and [ISA 3a4fa40](https://github.com/thecoshman/pir-8/blob/3a4fa40f4f716fc97a4bc432d06cf69f8cb9e0ce/ISA.md), used in v2.0.0:
  * `LOAD IMM WIDE`, which can load a two-byte immediate into `A&B`, `C&D`, `X&Y`, or `ADR`.
