# hm-asm
A small assembler I wrote for use with the microprocessor from the Book "Grundlagen der Technische Informatik" by Dirk
W. Hoffmann.

This microprocessor has a single accumulator register that calculations use.

## How does it work?
You can generate the data and program memory for a program like this:
```
$ cargo run -- generate examples/add_endless.asm
Data Memory:
0 1 1 0
0 0 0 0
0 0 0 0
0 0 0 0
Program Memory:
1 4 8 0
0 0 0 0
0 0 0 0
0 0 0 0
And that's your program!

```
Alternatively you can simulate an asm program for n clock cycles like this:
```
$ cargo run -- simulate examples/add_endless.asm 4
```
It is going to proceed and print an HTML table of all states since the only purpose of this tool is to avoid using
mahara as an in browser lab book -> we just autogenerate the tables.

## Syntax

The processor supports the following commands:

| Instruction | Coding | Description |
| ----------- |:------:| ----------- |
| `NOP`       | `0000` | No operation |
| `LDA #n`    | `0001` | Load value `n` to accumulator |
| `LDA (n)`   | `0010` | Load value from address `n` to accumulator |
| `STA (n)`    | `0011` | Store value from accumulator to address `n` |
| `ADD #n`    | `0100` | Add value `n` to accumulator |
| `ADD (n)`   | `0101` | Add value from address `n` to accumulator |
| `SUB #n`    | `0110` | Subtract value `n` from accumulator |
| `SUB (n)`   | `0111` | Subtract value from address `n` from accumulator |
| `JMP n`     | `1000` | Jump to program counter `n` |
| `BRZ #n`    | `1001` | Branch n instructions relative to current instruction when zero bit is set |
| `BRC #n`    | `1010` | Branch n instructions relative to current instruction when carry bit is set |
| `BRN #n`    | `1010` | Branch n instructions relative to current instruction when negative bit is set |

Each instruction is eight bit large: four bit for the opcode and four bit for the operand. The instruction is split into program memory (opcode) and data memory (operand).

Three state bits of the previous arithmetic operation can be used in control flow: `N`egative, `C`arry and `Z`ero.

You can find some syntax examples for the assembler in `examples/`

## Limitations
The operand size of four bits limits the directly addressable memory to 16 instructions. This generally limits the memory size, while large program memory is supported. Furthermore since the operands of the instructions are stored in the same part of memory as where you
can store values with the `STA` instruction it is technically possible to overwrite your program operands at runtime and do a
sort of self modifying programming style, this assembler does not warn you if you do this as of now.
