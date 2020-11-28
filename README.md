# hm-asm
A small assembler I wrote for use with the microprocessor from the Book "Grundlagen der Technische Informatik" by Dirk
W. Hoffmann.

## How does it work?
You run the binary with a path to an assembler file like this `cargo run path/to/file.asm`.

## Syntax?
You can find some syntax examples in `examples/`

## Limitations
Note that since this microprocessor is only a 4 bit microporcessor we cannot possibly have programs with more than 16
instructions. Furthermore since the parameters of the instructions are stored in the same part of memory as where you
can store values with the `STA` instruction it is technically possible to overwrite your program at runtime and do a
sort of self modifying programming style, this assembler does not warn you if you do this as of now.
