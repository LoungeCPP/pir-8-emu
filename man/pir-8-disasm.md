pir-8-disasm(1) -- Disassembler for the pir-8
=============================================

## SYNOPSIS

`pir-8-disasm` [OPTIONS] <FILE>

## DESCRIPTION

Disassembler for the pir-8.

Specified input file (or "-" for stdin) is disassembled into stdout.

## OPTIONS

  -e BYTES

    Skip BYTES bytes of header

  -k START,BYTES...

    Don't disassemble BYTES bytes from position START

    Can be specified multiple times

## Exit values

    1 - option parsing error
    2 - unused
    3 - input file opening failure
    4 - output write failure
    5 - input read failure
    6 - unused
    7 - insufficient instruction data

## EXAMPLES

  `pir-8-disasm test-data/xor-swap-with-loads.p8b`

    00000000   24   LOAD IND A
    00000002 0110 D 0x0110
    00000003   1D   LOAD IMM B
    00000004   69 D 0x69
    00000005   62   MOVE A X
    00000006   6B   MOVE B Y
    00000007   35   ALU XOR
    00000008   4C   MOVE S A
    00000009   63   MOVE A Y
    0000000A   6A   MOVE B X
    0000000B   35   ALU XOR
    0000000C   4D   MOVE S B
    0000000D   62   MOVE A X
    0000000E   6B   MOVE B Y
    0000000F   35   ALU XOR
    00000010   4C   MOVE S A
    00000011   FF   HALT

  `pir-8-disasm -e 3 test-data/xor-swap-with-loads.p8b`

    00000000   1D   LOAD IMM B
    00000001   69 D 0x69
    00000002   62   MOVE A X
    00000003   6B   MOVE B Y
    00000004   35   ALU XOR
    00000005   4C   MOVE S A
    00000006   63   MOVE A Y
    00000007   6A   MOVE B X
    00000008   35   ALU XOR
    00000009   4D   MOVE S B
    0000000A   62   MOVE A X
    0000000B   6B   MOVE B Y
    0000000C   35   ALU XOR
    0000000D   4C   MOVE S A
    0000000E   FF   HALT

  `pir-8-disasm -k 1,7 test-data/xor-swap-with-loads.p8b`

    00000000   24   LOAD IND A
    00000002 0110 D 0x0110
    00000003   1D   LOAD IMM B
    00000004   69 D 0x69
    00000005   62   MOVE A X
    00000006   6B   MOVE B Y
    00000007   35   ALU XOR
    00000008      S skipping 0x07 bytes
    00000010   4C   MOVE S A
    00000011   FF   HALT

  `pir-8-disasm -e 3 -k 1,0x0D test-data/xor-swap-with-loads.p8b`

    00000000   1D   LOAD IMM B
    00000001      S skipping 0x0D bytes
    0000000E   FF D 0xFF

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
