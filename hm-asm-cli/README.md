# hm-asm-cli

A CLI frontend for `hm-asm-simulate`, it provides two commands:

## Generate
You can generate the data and program memory for a program like this
```
$ cargo run -- compile ../examples/add_endless.asm
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

## Simulate
Alternatively you can simulate an asm program for n clock cycles like this:
```
$ cargo run -- simulate ../examples/add_endless.asm 4
```
It is going to proceed and print an HTML table of all states since the only purpose of this tool is to avoid using
mahara as an in browser lab book -> we just autogenerate the tables.
