# pir-8-emu [![TravisCI build status](https://travis-ci.com/LoungeCPP/pir-8-emu.svg?branch=master)](https://travis-ci.com/LoungeCPP/pir-8-emu) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/rg4hf7w778175wnt?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/pir-8-emu/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://meritbadge.herokuapp.com/pir-8-emu)](https://crates.io/crates/pir-8-emu)
Implementation of the [pir-8 ISA](https://github.com/thecoshman/pir-8).

## [Documentation](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/doc/pir_8_emu/index.html)
## [Manpage](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/man/pir-8-emu.1.html)

### Installation

#### From Crates.io

Start by obtaining Rust from https://rustup.rs, and [BearLibTerminal](https://bitbucket.org/cfyzium/bearlibterminal).
Afterwards, run

```sh
cargo install pir-8-emu
```

After the installation process finishes,
  move onto the [manpages](https://rawcdn.githack.com/LoungeCPP/pir-8-emu/man/pir-8-emu.1.html) to see how to emulate or {dis,}assemble your code.

If you've encountered a problem during the installation, do not hesitate to open an issue [here](https://github.com/LoungeCPP/pir-8-emu/issues/new).

#### From Debian repository

The following line in `/etc/apt/sources.list`:
```apt
deb https://debian.nabijaczleweli.xyz stable main
```

With [my PGP key](https://keybase.io/nabijaczleweli) (the two URLs are interchangeable):
```sh
wget -O- https://debian.nabijaczleweli.xyz/nabijaczleweli.gpg.key | sudo apt-key add
# or
sudo wget -O/etc/apt/trusted.gpg.d/nabijaczleweli.asc https://keybase.io/nabijaczleweli/pgp_keys.asc
```

Then the usual
```sh
sudo apt update
sudo apt install pir-8-emu pir-8-emu-binutils pir-8-emu-devel
```
will work on x86_64 and i686.

`pir-8-emu` includes the emulator, and is the only package that depends on `libbearlibterminal`, which is also included in the repository.<br />
`pir-8-emu-binutils` contains the {dis,}assembler, and<br />
`pir-8-emu-devel` has the example and devel header for emulated hardware components.

See the [repository README](https://debian.nabijaczleweli.xyz/README) for more information.

#### From pre-built executables

Alternatively, have a glance over at the [releases page](https://github.com/LoungeCPP/pir-8-emu/releases), which hosts Windows and Linux x86_64 binaries, including BLT.

Installation should be a matter of downloading and unpacking them, and copying somewhere to your `$PATH` and/or `$LD_LIBRARY_PATH`.

## Special thanks

To all who support further development on Patreon, in particular:

  * ThePhD
