pir-8-disasm(1) -- Disassembler for the pir-8
=============================================

## SYNOPSIS

`pir-8-disasm` [OPTIONS] <FILE>

## DESCRIPTION

Disassembler for the pir-8.

Specified input file (or "-" for stdin) is disassembled into stdout.

The output consists of four columns:

  * The leftmost 8 characters specify the address of the data in the input file,
  * The next 4 are the raw data, as read, right-aligned if the data is 1-byte wide,
  * The 1 character that follows functions as a status indicator, it can either be:

    - empty, if the data is an instruction,
    - an exclamation mark (!), if the instruction is invalid (reserved),
    - D, if the data is instruction data, or
    - S, if the line is a skip (-k) information

## OPTIONS

  -e BYTES

    Skip BYTES bytes of header

  -k START,BYTES...

    Don't disassemble BYTES bytes from position START

    Can be specified multiple times

  -r REGISTER_LETTERS

    Use REGISTER_LETTERS as the letters for the registers
    in the general-purpose bank instead of the defaults,
    as specified in the ISA

    Must be 8-ASCII-characters-long

## EXIT VALUES

    1 - option parsing error
    2 - unused
    3 - input file opening failure
    4 - output write failure
    5 - input read failure
    6 - unused
    7 - insufficient instruction data
    8 - unused
    9 - unused

## EXAMPLES

  `pir-8-disasm test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   11   LOAD IMM WIDE C&D
    00000002 001C D 0x001C
    00000003   7A   MOVE D X
    00000004   03   LOAD IMM BYTE Y
    00000005   01 D 0x01
    00000006   30   ALU ADD
    00000007   4F   MOVE S D
    00000008   72   MOVE C X
    00000009   03   LOAD IMM BYTE Y
    0000000A   00 D 0x00
    0000000B   32   ALU ADDC
    0000000C   4E   MOVE S C
    0000000D   D9   MADR WRITE C&D
    0000000E   04   LOAD IMM BYTE A
    0000000F   00 D 0x00
    00000010   0D   LOAD IND B
    00000011   E5   PORT OUT B
    00000012   69   MOVE B S
    00000013   F1   COMP S
    00000014   13   LOAD IMM WIDE ADR
    00000016 001C D 0x001C
    00000017   24   JMZG
    00000018   13   LOAD IMM WIDE ADR
    0000001A 0003 D 0x0003
    0000001B   27   JUMP
    0000001C   FF   HALT

  `pir-8-disasm -r 01234567 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   11   LOAD IMM WIDE C&D
    00000002 001C D 0x001C
    00000003   7A   MOVE 7 2
    00000004   03   LOAD IMM BYTE 3
    00000005   01 D 0x01
    00000006   30   ALU ADD
    00000007   4F   MOVE 1 7
    00000008   72   MOVE 6 2
    00000009   03   LOAD IMM BYTE 3
    0000000A   00 D 0x00
    0000000B   32   ALU ADDC
    0000000C   4E   MOVE 1 6
    0000000D   D9   MADR WRITE C&D
    0000000E   04   LOAD IMM BYTE 4
    0000000F   00 D 0x00
    00000010   0D   LOAD IND 5
    00000011   E5   PORT OUT 5
    00000012   69   MOVE 5 1
    00000013   F1   COMP 1
    00000014   13   LOAD IMM WIDE ADR
    00000016 001C D 0x001C
    00000017   24   JMZG
    00000018   13   LOAD IMM WIDE ADR
    0000001A 0003 D 0x0003
    0000001B   27   JUMP
    0000001C   FF   HALT

  `pir-8-disasm -e 3 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   7A   MOVE D X
    00000001   03   LOAD IMM BYTE Y
    00000002   01 D 0x01
    00000003   30   ALU ADD
    00000004   4F   MOVE S D
    00000005   72   MOVE C X
    00000006   03   LOAD IMM BYTE Y
    00000007   00 D 0x00
    00000008   32   ALU ADDC
    00000009   4E   MOVE S C
    0000000A   D9   MADR WRITE C&D
    0000000B   04   LOAD IMM BYTE A
    0000000C   00 D 0x00
    0000000D   0D   LOAD IND B
    0000000E   E5   PORT OUT B
    0000000F   69   MOVE B S
    00000010   F1   COMP S
    00000011   13   LOAD IMM WIDE ADR
    00000013 001C D 0x001C
    00000014   24   JMZG
    00000015   13   LOAD IMM WIDE ADR
    00000017 0003 D 0x0003
    00000018   27   JUMP
    00000019   FF   HALT

  `pir-8-disasm -k 0x14,7 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   11   LOAD IMM WIDE C&D
    00000002 001C D 0x001C
    00000003   7A   MOVE D X
    00000004   03   LOAD IMM BYTE Y
    00000005   01 D 0x01
    00000006   30   ALU ADD
    00000007   4F   MOVE S D
    00000008   72   MOVE C X
    00000009   03   LOAD IMM BYTE Y
    0000000A   00 D 0x00
    0000000B   32   ALU ADDC
    0000000C   4E   MOVE S C
    0000000D   D9   MADR WRITE C&D
    0000000E   04   LOAD IMM BYTE A
    0000000F   00 D 0x00
    00000010   0D   LOAD IND B
    00000011   E5   PORT OUT B
    00000012   69   MOVE B S
    00000013   F1   COMP S
    00000014      S skipping 0x07 bytes
    0000001B   27   JUMP
    0000001C   FF   HALT

  `pir-8-disasm -e 3 -k 17,0b111 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   7A   MOVE D X
    00000001   03   LOAD IMM BYTE Y
    00000002   01 D 0x01
    00000003   30   ALU ADD
    00000004   4F   MOVE S D
    00000005   72   MOVE C X
    00000006   03   LOAD IMM BYTE Y
    00000007   00 D 0x00
    00000008   32   ALU ADDC
    00000009   4E   MOVE S C
    0000000A   D9   MADR WRITE C&D
    0000000B   04   LOAD IMM BYTE A
    0000000C   00 D 0x00
    0000000D   0D   LOAD IND B
    0000000E   E5   PORT OUT B
    0000000F   69   MOVE B S
    00000010   F1   COMP S
    00000011      S skipping 0x07 bytes
    00000018   27   JUMP
    00000019   FF   HALT

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
