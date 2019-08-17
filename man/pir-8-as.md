pir-8-as(1) -- Assembler for the pir-8
======================================

## SYNOPSIS

`pir-8-as` [OPTIONS] [ASMFILE...]

## DESCRIPTION

Assembler for the pir-8.

Specified input files (or "-" for stdin) are assembled sequentially into the output file.

The comment characer is ";".

The assembly format is as follows:

  * 1 instruction per line, or
  * if an instruction required data, each line afterward
    will be attempted to be parsed as data therefor.

And so, all of these are equivalent:

    LOAD IND A
    0x0110

    LOAD IND A
    0b0000_0001_0001_0000

    0x24
    0o420

    36
    272

## DIRECTIVES

Start with a colon, not limited to ASCII:

  :origin <ADDRESS>

    The first instruction starts at ADDRESS, all previous bytes are zeroed

    Using this more than once or after having already processed an instruction will yield an error

  :label save <NAME>

    Save the current output address to be recalled anywhere else in the program assembly

  :label load <NAME>

    Substitute the output address of previously saved label called NAME in this place

    If the NAME label wasn't yet specified, output will be buffered until it's declared

    Using this when the current instruction isn't expecting two data bytes will yield an error

    Having specified this with a NAME without a corresponding :label save directive will yield an error

  :label load-offset <NAME> <OFFSET>

    Like :label load NAME, but add (signed) OFFSET afterwards

    The resulting address will wrap around both sides

  :literal "<STRING>"

    Insert STRING into the output

    Using this when the current instruction is expecting data will yield an error

## OPTIONS

  -o BINFILE

    Name of the the binary-file output, or "-" for stdout

    Parent directory must exist

    Default: "a.p8b"

  -r REGISTER_LETTERS

    Use REGISTER_LETTERS as the letters for the registers
    in the general-purpose bank instead of the defaults,
    as specified in the ISA

    Must be 8-ASCII-characters-long

## EXIT VALUES

    1 - option parsing error
    2 - output file creation failure
    3 - input file opening failure
    4 - output write failure
    5 - input read failure
    6 - instruction parse error
    7 - instruction data parse error
    8 - invalid directive or directive obey error
    9 - unfound labels remain

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
