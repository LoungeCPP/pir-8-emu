pir-8-as(1) -- Assembler for the pir-8
======================================

## SYNOPSIS

`pir-8-as` [OPTIONS] [ASMFILE...]

## DESCRIPTION

Assembler for the pir-8.

Specified input files (or "-" for stdin) are assembled sequentially into the output file.

The comment characer is ";".

## OPTIONS

  -o BINFILE

    Name of the the binary-file output, or "-" for stdout

    Parent directory must exist

    Default: "a.p8b"

## Exit values

    1 - option parsing error
    2 - output file creation failure
    3 - input file opening failure
    4 - output write failure
    5 - input read failure
    6 - instruction parse error
    7 - instruction data parse error

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
