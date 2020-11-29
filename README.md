# hm-asm
A small assembler I wrote for use with the microprocessor from the Book "Grundlagen der Technische Informatik" by Dirk
W. Hoffmann.

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

## Syntax?
You can find some syntax examples for the assembler in `examples/`

## Limitations
Note that since this microprocessor is only a 4 bit microporcessor we cannot possibly have programs with more than 16
instructions. Furthermore since the parameters of the instructions are stored in the same part of memory as where you
can store values with the `STA` instruction it is technically possible to overwrite your program at runtime and do a
sort of self modifying programming style, this assembler does not warn you if you do this as of now.
