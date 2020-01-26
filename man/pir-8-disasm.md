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

    00000000   1E   LOAD IMM C
    00000001   00 D 0x00
    00000002   1A   LOAD IMM X
    00000003   27 D 0x27
    00000004   1B   LOAD IMM Y
    00000005   01 D 0x01
    00000006   31   ALU SUB
    00000007   4A   MOVE S X
    00000008   4F   MOVE S D
    00000009   7A   MOVE D X
    0000000A   1B   LOAD IMM Y
    0000000B   01 D 0x01
    0000000C   30   ALU ADD
    0000000D   4F   MOVE S D
    0000000E   72   MOVE C X
    0000000F   1B   LOAD IMM Y
    00000010   00 D 0x00
    00000011   32   ALU ADDC
    00000012   4E   MOVE S C
    00000013   0D   MADR WRITE C&D
    00000014   1C   LOAD IMM A
    00000015   00 D 0x00
    00000016   25   LOAD IND B
    00000017   E5   PORT OUT B
    00000018   69   MOVE B S
    00000019   F1   COMP S
    0000001A   1C   LOAD IMM A
    0000001B   00 D 0x00
    0000001C   1D   LOAD IMM B
    0000001D   26 D 0x26
    0000001E   0C   MADR WRITE A&B
    0000001F   14   JMZG
    00000020   1C   LOAD IMM A
    00000021   00 D 0x00
    00000022   1D   LOAD IMM B
    00000023   09 D 0x09
    00000024   0C   MADR WRITE A&B
    00000025   17   JUMP
    00000026   FF   HALT

  `pir-8-disasm -r 01234567 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   1E   LOAD IMM 6
    00000001   00 D 0x00
    00000002   1A   LOAD IMM 2
    00000003   27 D 0x27
    00000004   1B   LOAD IMM 3
    00000005   01 D 0x01
    00000006   31   ALU SUB
    00000007   4A   MOVE 1 2
    00000008   4F   MOVE 1 7
    00000009   7A   MOVE 7 2
    0000000A   1B   LOAD IMM 3
    0000000B   01 D 0x01
    0000000C   30   ALU ADD
    0000000D   4F   MOVE 1 7
    0000000E   72   MOVE 6 2
    0000000F   1B   LOAD IMM 3
    00000010   00 D 0x00
    00000011   32   ALU ADDC
    00000012   4E   MOVE 1 6
    00000013   0D   MADR WRITE C&D
    00000014   1C   LOAD IMM 4
    00000015   00 D 0x00
    00000016   25   LOAD IND 5
    00000017   E5   PORT OUT 5
    00000018   69   MOVE 5 1
    00000019   F1   COMP 1
    0000001A   1C   LOAD IMM 4
    0000001B   00 D 0x00
    0000001C   1D   LOAD IMM 5
    0000001D   26 D 0x26
    0000001E   0C   MADR WRITE A&B
    0000001F   14   JMZG
    00000020   1C   LOAD IMM 4
    00000021   00 D 0x00
    00000022   1D   LOAD IMM 5
    00000023   09 D 0x09
    00000024   0C   MADR WRITE A&B
    00000025   17   JUMP
    00000026   FF   HALT

  `pir-8-disasm -e 9 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   7A   MOVE D X
    00000001   1B   LOAD IMM Y
    00000002   01 D 0x01
    00000003   30   ALU ADD
    00000004   4F   MOVE S D
    00000005   72   MOVE C X
    00000006   1B   LOAD IMM Y
    00000007   00 D 0x00
    00000008   32   ALU ADDC
    00000009   4E   MOVE S C
    0000000A   0D   MADR WRITE C&D
    0000000B   1C   LOAD IMM A
    0000000C   00 D 0x00
    0000000D   25   LOAD IND B
    0000000E   E5   PORT OUT B
    0000000F   69   MOVE B S
    00000010   F1   COMP S
    00000011   1C   LOAD IMM A
    00000012   00 D 0x00
    00000013   1D   LOAD IMM B
    00000014   26 D 0x26
    00000015   0C   MADR WRITE A&B
    00000016   14   JMZG
    00000017   1C   LOAD IMM A
    00000018   00 D 0x00
    00000019   1D   LOAD IMM B
    0000001A   09 D 0x09
    0000001B   0C   MADR WRITE A&B
    0000001C   17   JUMP
    0000001D   FF   HALT

  `pir-8-disasm -k 0x1A,11 test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   1E   LOAD IMM C
    00000001   00 D 0x00
    00000002   1A   LOAD IMM X
    00000003   27 D 0x27
    00000004   1B   LOAD IMM Y
    00000005   01 D 0x01
    00000006   31   ALU SUB
    00000007   4A   MOVE S X
    00000008   4F   MOVE S D
    00000009   7A   MOVE D X
    0000000A   1B   LOAD IMM Y
    0000000B   01 D 0x01
    0000000C   30   ALU ADD
    0000000D   4F   MOVE S D
    0000000E   72   MOVE C X
    0000000F   1B   LOAD IMM Y
    00000010   00 D 0x00
    00000011   32   ALU ADDC
    00000012   4E   MOVE S C
    00000013   0D   MADR WRITE C&D
    00000014   1C   LOAD IMM A
    00000015   00 D 0x00
    00000016   25   LOAD IND B
    00000017   E5   PORT OUT B
    00000018   69   MOVE B S
    00000019   F1   COMP S
    0000001A      S skipping 0x0B bytes
    00000025   17   JUMP
    00000026   FF   HALT

  `pir-8-disasm -e 3 -k 1,0x0D test-data/copy-any-length-literal-to-port-nolit.p8b`

    00000000   7A   MOVE D X
    00000001   1B   LOAD IMM Y
    00000002   01 D 0x01
    00000003   30   ALU ADD
    00000004   4F   MOVE S D
    00000005   72   MOVE C X
    00000006   1B   LOAD IMM Y
    00000007   00 D 0x00
    00000008   32   ALU ADDC
    00000009   4E   MOVE S C
    0000000A   0D   MADR WRITE C&D
    0000000B   1C   LOAD IMM A
    0000000C   00 D 0x00
    0000000D   25   LOAD IND B
    0000000E   E5   PORT OUT B
    0000000F   69   MOVE B S
    00000010   F1   COMP S
    00000011      S skipping 0x0B bytes
    0000001C   17   JUMP
    0000001D   FF   HALT

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/LoungeCPP/pir-8-emu/issues>&gt;

## SEE ALSO

&lt;<https://github.com/LoungeCPP/pir-8-emu>&gt;
